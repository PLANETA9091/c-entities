//! The persistent AoS mob registry + the Seen grid (Horizon-style).
//!
//! Horizon's server keeps mobs as a plain `Vec<Mob>` in insertion order with
//! a `FxHashMap<u32,u32>` id→slot cache and a per-tick `Seen` snapshot
//! indexed by a 4.0-block uniform XZ grid (`SeenGrid::candidates(area)` is a
//! guaranteed superset). This module ports that shape: `MobStore` owns the
//! population, `SeenGrid` answers spatial candidate queries, vanilla
//! insertion order is preserved by slot recycling (freed slots go to a free
//! list ONLY at the tail; interior removals tombstone to keep iteration
//! order stable within the tick — the host removes between ticks).
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use swarm_sim::mob::MobAI;

use crate::world::Snapshot;

/// CELL in blocks — Horizon seen_grid.rs:8 (`CELL=4.0`).
pub const GRID_CELL: f64 = 4.0;

/// One registry entry: the parity-proven per-mob state (`MobAI` — goal
/// records, RNG mirror, nav, physics f64 fields) plus the registry plumbing
/// Horizon carries on the struct (join order, alive flag, grid slot).
/// WAVE14: Clone — the parallel double-buffer swaps `mobs` with a
/// shadow Vec of slots each tick (the store's post-tick buffer becomes
/// the working one for the next tick; no per-tick deep copy pass).
#[derive(Clone)]
pub struct MobSlot {
    pub mob: MobAI,
    /// alive=false entries are tombstones awaiting the between-tick compact.
    pub alive: bool,
    /// grid bucket index ((i64::MAX, i64::MAX) = unbuilt), refreshed per tick.
    pub grid_cell: (i64, i64),
    /// WAVE8b: the slot's index WITHIN its cell's bucket vec — the O(1)
    /// detach handle (swap_remove + the swapped element's re-stamp). Stale
    /// whenever grid_cell is the unbuilt sentinel; guarded at every use.
    pub grid_pos: u32,
}

/// WAVE8b: the cells-map hasher — FxHash-style multiply-xor over the key
/// words. The default RandomState (SipHash-1-3) cost ~25ns per map op;
/// every mover pays TWO per tick (the old-cell detach + the new-cell
/// insert) ≈ 0.9ms/tick at the 25k pile. The keys are internal (never
/// exposed across the wire), so a fixed-seed cheap hash is safe — Horizon's
/// own grid uses FxHashMap for the same reason.
#[derive(Default)]
pub struct CellHasher {
    h: u64,
}

impl Hasher for CellHasher {
    #[inline]
    fn write_u64(&mut self, w: u64) {
        self.h = (self.h.rotate_left(5) ^ w).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    #[inline]
    fn write_i64(&mut self, w: i64) {
        self.write_u64(w as u64);
    }
    #[inline]
    fn write_u32(&mut self, w: u32) {
        self.write_u64(w as u64);
    }
    #[inline]
    fn write_u8(&mut self, w: u8) {
        self.write_u64(w as u64);
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut chunks = bytes.chunks_exact(8);
        for c in &mut chunks {
            self.write_u64(u64::from_le_bytes(c.try_into().unwrap()));
        }
        let rem = chunks.remainder();
        if !rem.is_empty() {
            let mut buf = [0u8; 8];
            buf[..rem.len()].copy_from_slice(rem);
            self.write_u64(u64::from_le_bytes(buf));
        }
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.h
    }
}

type CellMap = HashMap<(i64, i64), Vec<u32>, BuildHasherDefault<CellHasher>>;

/// The uniform 2-D XZ grid over mob slots. Cells are `HashMap<(i64,i64),
/// Vec<u32>)` — at 25k mobs in a pile the hot cells hold thousands of slots;
/// the candidates() contract is a SUPERSET (never misses a mob whose box
/// touches the area), which is all the floor's queries need (they re-test
/// exact boxes themselves).
#[derive(Default)]
pub struct SeenGrid {
    cells: CellMap,
    pub population: u64,
    /// WAVE8c: lazy maintenance — the grid rebuilds (full, from live
    /// positions) on the first query after any population/position change.
    /// The native v0 tick never queries (the folds answer scans natively),
    /// so the per-tick re-bucket walk was pure overhead. The content is
    /// only observable through candidates()/cell_len — both fire the
    /// rebuild first, so the superset contract is unchanged bit-for-bit.
    pub dirty: bool,
}

impl SeenGrid {
    /// WAVE2 strength reduction: `x / 4.0` → `x * 0.25`. Division by a
    /// power of two is EXACT in IEEE-754 (exponent decrement; both ops
    /// round the same real quotient identically, normal AND subnormal),
    /// so `floor(x * 0.25) == x.div_euclid(4.0)` bit-for-bit — same cell,
    /// zero div-unit pressure (the rebuild walk used to burn 2 fdiv per
    /// mob per tick; the sim-grid hot cells were measured at 1.8 ms/tick).
    #[inline]
    pub fn cell_of(x: f64, z: f64) -> (i64, i64) {
        // 1/4.0 == 0.25 is exact (power of two), so this folds to the
        // documented bit-exact multiply.
        ((x * (1.0 / GRID_CELL)).floor() as i64, (z * (1.0 / GRID_CELL)).floor() as i64)
    }

    /// WAVE2 — the Horizon N1 optimization (their seen_grid update(i,b)):
    /// INCREMENTAL re-bucket. The per-tick full rebuild cleared the map
    /// and re-hashed every mob (25k hash inserts/tick); the superset
    /// contract only needs every alive mob listed under the cell of its
    /// CURRENT position, so: walk the slots once (no hashing), re-bucket
    /// ONLY slots whose stored cell ≠ computed cell (the movers), leave
    /// the rest untouched. Cells persist across ticks; stale tombstone
    /// entries die at detach-on-remove + the compact invalidation.
    ///
    /// Correctness: `rebuild` runs BEFORE the tick's AI, and positions
    /// only change in travel (after it) — so at call time x/z ARE the
    /// latest positions and the compare-to-`grid_cell` check re-buckets
    /// every mover. A mob is never missing: unbuilt slots insert, moved
    /// slots detach + insert, stationary slots are already right.
    pub fn rebuild(&mut self, mobs: &mut [MobSlot]) {
        let mut population = 0u64;
        for slot_i in 0..mobs.len() {
            if !mobs[slot_i].alive {
                continue;
            }
            population += 1;
            let c = Self::cell_of(mobs[slot_i].mob.x, mobs[slot_i].mob.z);
            if mobs[slot_i].grid_cell == c {
                continue; // stationary: already listed under this cell
            }
            let old_cell = mobs[slot_i].grid_cell;
            // detach from the stale cell (order-insensitive: the
            // candidates() contract is an unordered superset).
            // WAVE8b: the slot's position rides grid_pos — the O(len)
            // position() scan is gone (the 25k pile's hot cells held ~35
            // slots: ~17 linear comparisons per mover per tick).
            if old_cell.0 != i64::MAX {
                if let Some(old) = self.cells.get_mut(&old_cell) {
                    let pos = mobs[slot_i].grid_pos as usize;
                    if pos < old.len() && old[pos] == slot_i as u32 {
                        old.swap_remove(pos);
                        if pos < old.len() {
                            // the element swapped into `pos` re-stamps
                            let moved = old[pos] as usize;
                            mobs[moved].grid_pos = pos as u32;
                        }
                    } else {
                        // invariant broke (a detached-but-not-rebuilt slot
                        // would land here): the honest linear fallback
                        if let Some(pos) = old.iter().position(|&s| s as usize == slot_i) {
                            old.swap_remove(pos);
                        }
                    }
                }
            }
            let v = self.cells.entry(c).or_default();
            v.push(slot_i as u32);
            mobs[slot_i].grid_pos = (v.len() - 1) as u32;
            mobs[slot_i].grid_cell = c;
        }
        self.population = population;
        self.dirty = false;
    }

    /// Detach a slot from its listed cell (remove()-time; the tombstone
    /// must not linger as a stale candidate and slot recycling must not
    /// double-list). No-op for unbuilt slots.
    /// WAVE8b: the mobs slice rides along for the swapped element's
    /// grid_pos re-stamp (the O(1) handle must stay exact).
    pub fn detach(&mut self, slot: u32, cell: (i64, i64), mobs: &mut [MobSlot]) {
        if cell.0 == i64::MAX {
            return;
        }
        if let Some(old) = self.cells.get_mut(&cell) {
            let pos = mobs[slot as usize].grid_pos as usize;
            if pos < old.len() && old[pos] == slot {
                old.swap_remove(pos);
                if pos < old.len() {
                    let moved = old[pos] as usize;
                    mobs[moved].grid_pos = pos as u32;
                }
            } else if let Some(pos) = old.iter().position(|&s| s == slot) {
                old.swap_remove(pos);
            }
        }
        mobs[slot as usize].grid_pos = u32::MAX;
    }

    /// Full invalidation (the compact renumbering): every stored cell
    /// index is stale, the next rebuild re-inserts everything.
    pub fn invalidate_all(&mut self) {
        self.cells.clear();
        self.population = 0;
        self.dirty = true;
    }

    /// A superset of the mobs whose position could touch the XZ area.
    /// Returned slot indices are unordered (Horizon contract: the caller
    /// re-tests exact boxes; order-sensitive callers sort by slot).
    /// WAVE8c: LAZY — a dirty grid rebuilds (full walk, live positions)
    /// before answering; clean grids answer from the maintained buckets.
    pub fn candidates(&mut self, mobs: &mut [MobSlot], min_x: f64, min_z: f64, max_x: f64, max_z: f64, out: &mut Vec<u32>) {
        if self.dirty {
            self.rebuild(mobs);
            self.dirty = false;
        }
        out.clear();
        let cx0 = (min_x * 0.25).floor() as i64;
        let cx1 = (max_x * 0.25).floor() as i64;
        let cz0 = (min_z * 0.25).floor() as i64;
        let cz1 = (max_z * 0.25).floor() as i64;
        // the overflow guard: an area spanning too many columns falls back
        // to a full walk of the cells map (Horizon seen_grid.rs:69)
        let span_x = (cx1 - cx0).saturating_add(1) as usize;
        let span_z = (cz1 - cz0).saturating_add(1) as usize;
        if span_x.saturating_mul(span_z) > 1024 {
            for cell in self.cells.values() {
                out.extend_from_slice(cell);
            }
            return;
        }
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                if let Some(cell) = self.cells.get(&(cx, cz)) {
                    out.extend_from_slice(cell);
                }
            }
        }
    }

    #[inline]
    pub fn cell_len(&mut self, mobs: &mut [MobSlot], cx: i64, cz: i64) -> usize {
        if self.dirty {
            self.rebuild(mobs);
            self.dirty = false;
        }
        self.cells.get(&(cx, cz)).map_or(0, |c| c.len())
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.population = 0;
        self.dirty = true;
    }
}

/// The persistent registry.
pub struct MobStore {
    pub mobs: Vec<MobSlot>,
    /// id -> slot in `mobs` (Horizon mob_index.rs:18 — the cache is
    /// authoritative because slots only move at compaction time).
    pub index: HashMap<i64, u32>,
    /// tombstone slots awaiting compaction (pub: the executor checks the
    /// threshold; add() recycles from the tail of the free list)
    pub free: Vec<u32>,
    /// The next vanilla-sequential id the host should hand out (ids are
    /// server-global; the store only mirrors them).
    pub game_time: i32,
    /// The host-pushed per-tick state (players). Borrowed immutably by the
    /// resolver while the mobs mutate.
    pub snapshot: Snapshot,
}

impl MobStore {
    pub fn new() -> Self {
        MobStore {
            mobs: Vec::new(),
            index: HashMap::new(),
            free: Vec::new(),
            game_time: 0,
            snapshot: Snapshot::default(),
        }
    }

    /// Insert (vanilla insertion order: tail append; recycled slots only
    /// when the free list is non-empty — recycled slots preserve the
    /// relative order because removals happen between ticks).
    pub fn add(&mut self, mob: MobAI) {
        let id = mob.id;
        if self.index.contains_key(&id) {
            return; // the host must not double-push an id
        }
        let slot = match self.free.pop() {
            Some(s) => {
                self.mobs[s as usize] = MobSlot {
                    mob,
                    alive: true,
                    grid_cell: (i64::MAX, i64::MAX),
            grid_pos: u32::MAX,
                };
                s
            }
            None => {
                self.mobs.push(MobSlot {
                    mob,
                    alive: true,
                    grid_cell: (i64::MAX, i64::MAX),
            grid_pos: u32::MAX,
                });
                (self.mobs.len() - 1) as u32
            }
        };
        self.index.insert(id, slot);
    }

    /// Remove by id (between ticks). Tombstones compact lazily: `compact`
    /// runs when tombstones exceed a quarter of the population. Returns
    /// the detached slot (the caller detaches it from the SeenGrid).
    pub fn remove(&mut self, id: i64) -> Option<u32> {
        let slot = self.index.remove(&id)?;
        self.mobs[slot as usize].alive = false;
        self.free.push(slot);
        Some(slot)
    }

    pub fn compact(&mut self) {
        if self.free.is_empty() && self.mobs.iter().all(|m| m.alive) {
            return;
        }
        // keep only alive, remember the old->new slot map, rebuild index
        let mut new_vec = Vec::with_capacity(self.mobs.len());
        let mut map: HashMap<i64, u32> = HashMap::with_capacity(self.index.len());
        for m in self.mobs.drain(..) {
            if m.alive {
                map.insert(m.mob.id, new_vec.len() as u32);
                new_vec.push(m);
            }
        }
        self.mobs = new_vec;
        self.index = map;
        self.free.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.index.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn clear(&mut self) {
        self.mobs.clear();
        self.index.clear();
        self.free.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swarm_sim::mob::zombie_table;

    fn mk(id: i64, x: f64, z: f64) -> MobAI {
        let mut m = MobAI::new(id, 0x1234_5678_9abc_def0, x, 64.0, z);
        m.goals = swarm_sim::mob::zombie_table_res();
        m
    }

    #[test]
    fn add_remove_reinsert_keeps_vanilla_order() {
        let mut s = MobStore::new();
        s.add(mk(1, 0.0, 0.0));
        s.add(mk(2, 1.0, 0.0));
        s.add(mk(3, 2.0, 0.0));
        s.remove(2);
        s.compact();
        s.add(mk(4, 3.0, 0.0));
        // vanilla order: 1, 3, 4
        let ids: Vec<i64> = s.mobs.iter().map(|m| m.mob.id).collect();
        assert_eq!(ids, vec![1, 3, 4]);
        assert_eq!(s.len(), 3);
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(s.index[id], i as u32);
        }
    }

    #[test]
    fn grid_candidates_superset_and_overflow_fallback() {
        let mut s = MobStore::new();
        s.add(mk(1, 0.0, 0.0));
        s.add(mk(2, 3.9, 0.0));
        s.add(mk(3, 4.1, 0.0));
        let mut g = SeenGrid::default();
        g.rebuild(&mut s.mobs);
        let mut out = Vec::new();
        g.candidates(&mut s.mobs, -1.0, -1.0, 3.5, 1.0, &mut out);
        out.sort_unstable();
        assert_eq!(out, vec![0, 1]); // 4.1 is in the next cell — must NOT be missed by [0,3.5]
        // degenerate far-apart area: overflow fallback walks all cells
        g.candidates(&mut s.mobs, -1.0e6, -1.0e6, 1.0e6, 1.0e6, &mut out);
        assert_eq!(out.len(), 3);
        // double-check the boundary: an area touching the boundary of a cell
        // must include the neighbour mob at 4.1 (superset, not exact)
        g.candidates(&mut s.mobs, 0.0, 0.0, 4.0, 0.0, &mut out);
        out.sort_unstable();
        assert!(out.contains(&2));
    }

    /// WAVE2 gate: the INCREMENTAL rebuild must keep the superset
    /// contract — a moved mob is found around its NEW position on the
    /// next rebuild, removed mobs vanish, a recycled slot is not
    /// double-listed under its old cell.
    #[test]
    fn incremental_rebuild_tracks_movers_and_recycling() {
        let mut s = MobStore::new();
        s.add(mk(1, 0.0, 0.0));
        s.add(mk(2, 8.0, 8.0));
        s.add(mk(3, 100.0, 100.0));
        let mut g = SeenGrid::default();
        g.rebuild(&mut s.mobs);
        let mut out = Vec::new();
        // both far cells list their own mob
        g.candidates(&mut s.mobs, 7.9, 7.9, 8.1, 8.1, &mut out);
        assert_eq!(out, vec![1]);
        // mob 2 walks three cells over (8,8 -> 20,8): cross-cell move
        s.mobs[1].mob.x = 20.0;
        g.rebuild(&mut s.mobs);
        g.candidates(&mut s.mobs, 7.5, 7.5, 8.5, 8.5, &mut out);
        assert!(out.is_empty(), "the old cell must not list a mover");
        g.candidates(&mut s.mobs, 19.9, 7.9, 20.1, 8.1, &mut out);
        assert_eq!(out, vec![1], "the new cell must list the mover");
        // stationary mob: untouched by the second rebuild
        g.candidates(&mut s.mobs, -0.1, -0.1, 0.1, 0.1, &mut out);
        assert_eq!(out, vec![0]);
        // remove detaches; recycling the slot must not double-list it
        let slot = s.remove(2).expect("removed");
        let cell = s.mobs[slot as usize].grid_cell;
        g.detach(slot, cell, &mut s.mobs);
        g.candidates(&mut s.mobs, 19.5, 7.5, 20.5, 8.5, &mut out);
        assert!(out.is_empty(), "removed mob must leave the grid");
        // slot 1 recycles as a NEW mob near (21,8): rebuild re-lists it,
        // exactly once
        let m4 = mk(4, 21.0, 8.0);
        s.mobs[1] = crate::store::MobSlot {
            mob: m4,
            alive: true,
            grid_cell: (i64::MAX, i64::MAX),
            grid_pos: u32::MAX,
        };
        s.index.insert(4, 1);
        s.free.pop();
        g.rebuild(&mut s.mobs);
        g.candidates(&mut s.mobs, 20.5, 7.5, 21.5, 8.5, &mut out);
        assert_eq!(out, vec![1], "recycled slot listed once at its new cell");
        // compact renumbering invalidates; the rebuild rebuilds all
        s.compact();
        g.invalidate_all();
        for m in &mut s.mobs {
            m.grid_cell = (i64::MAX, i64::MAX);
        }
        g.rebuild(&mut s.mobs);
        g.candidates(&mut s.mobs, -1.0, -1.0, 200.0, 200.0, &mut out);
        out.sort_unstable();
        assert_eq!(out.len(), 3, "all alive mobs after compact+rebuild");
    }

    /// WAVE2 arithmetic gate: the strength-reduced cell_of
    /// (`x * 0.25` + floor) must equal `x.div_euclid(4.0)` EXACTLY over a
    /// dense sweep including negatives, cell boundaries, huge magnitudes
    /// and the f64 subnormal fringe (the contract that lets the div go).
    #[test]
    fn cell_of_matches_div_euclid_bit_exact() {
        let mut x = -4096.0f64;
        while x <= 4096.0 {
            for z in [-4095.5f64, -4.0, -3.9, -0.25, -0.0, 0.0, 0.25, 3.9, 4.0, 4095.5] {
                let want = (x.div_euclid(4.0) as i64, z.div_euclid(4.0) as i64);
                let got = SeenGrid::cell_of(x, z);
                assert_eq!(got, want, "cell_of({x}, {z})");
            }
            x += 0.125;
        }
        for v in [
            -1.0e300, -1.0e18, -65_536.0, -4.5, -4.0, -1e-300, 1e-300, 4.0, 4.5,
            65_536.0, 1.0e18, 1.0e300, f64::MAX, f64::MIN, f64::INFINITY,
        ] {
            let want = (v.div_euclid(4.0) as i64, v.div_euclid(4.0) as i64);
            assert_eq!(SeenGrid::cell_of(v, v), want, "cell_of({v})");
        }
    }

    #[test]
    fn double_add_is_ignored() {
        let mut s = MobStore::new();
        s.add(mk(7, 0.0, 0.0));
        s.add(mk(7, 9.0, 9.0));
        assert_eq!(s.len(), 1);
        assert!((s.mobs[0].mob.x - 0.0).abs() < 1e-9);
    }
}

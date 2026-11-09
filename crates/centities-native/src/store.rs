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

use swarm_sim::mob::MobAI;

use crate::world::Snapshot;

/// CELL in blocks — Horizon seen_grid.rs:8 (`CELL=4.0`).
pub const GRID_CELL: f64 = 4.0;

/// One registry entry: the parity-proven per-mob state (`MobAI` — goal
/// records, RNG mirror, nav, physics f64 fields) plus the registry plumbing
/// Horizon carries on the struct (join order, alive flag, grid slot).
pub struct MobSlot {
    pub mob: MobAI,
    /// alive=false entries are tombstones awaiting the between-tick compact.
    pub alive: bool,
    /// grid bucket index (usize::MAX = unbuilt), refreshed per tick.
    pub grid_cell: (i64, i64),
}

/// The uniform 2-D XZ grid over mob slots — the L111 flat rewrite. No
/// HashMap, no per-cell Vec: one open-addressed table (zigzag-packed u64
/// cell keys, murmur finalizer, linear probing, power-of-two capacity),
/// occupied slots stamped with a per-rebuild GENERATION (no per-tick
/// memset — stale entries are dead by stamp, not by clearing), parallel
/// (start,len) ranges into ONE contiguous slot-id Vec built by a
/// counting-scatter (three cache-sequential passes). Every steady-state
/// cost is a multiply, a shift, a mask or a contiguous walk.
///
/// The candidates() contract is unchanged: a SUPERSET of the mobs whose
/// position could touch the XZ area (the floor's queries re-test exact
/// boxes themselves); cell contents are unordered.
pub struct SeenGrid {
    /// open-addressed cell keys (valid only where gens == cur_gen)
    keys: Vec<u64>,
    /// per-slot generation stamp (0 = never used; 0 is never a cur_gen)
    gens: Vec<u32>,
    /// per-slot (start, len) into `vals` (valid where gens == cur_gen)
    rngs: Vec<(u32, u32)>,
    /// per-slot scatter cursor (pass 3 scratch)
    cursor: Vec<u32>,
    /// capacity - 1 (power of two)
    mask: usize,
    /// current generation stamp
    cur_gen: u32,
    /// compact list of occupied table slots for the current generation
    occ: Vec<u32>,
    /// flat slot-id list, per-cell contiguous
    vals: Vec<u32>,
    /// per-mob cell keys + table slots (pass scratch, parallel to the
    /// triples slice)
    kbuf: Vec<u64>,
    tmp_cell: Vec<u32>,
    pub population: u64,
}

impl Default for SeenGrid {
    fn default() -> Self {
        SeenGrid {
            keys: Vec::new(),
            gens: Vec::new(),
            rngs: Vec::new(),
            cursor: Vec::new(),
            mask: 0,
            cur_gen: 1,
            occ: Vec::new(),
            vals: Vec::new(),
            kbuf: Vec::new(),
            tmp_cell: Vec::new(),
            population: 0,
        }
    }
}

/// murmur3-64 finalizer — multiply-xor mix, no tables, no branches.
#[inline]
fn cell_hash(k: u64) -> u64 {
    let mut h = k;
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    h ^= h >> 32;
    h
}

#[inline]
fn zigzag(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

impl SeenGrid {
    /// floor(v / 4.0) — the multiply by 0.25 is an EXACT power-of-two
    /// scale (IEEE-754: x*0.25 and x/4.0 are the same correctly-rounded
    /// value), so this is bit-identical to `v.div_euclid(GRID_CELL)` at a
    /// fraction of the latency (no divide unit, no floor libm call on
    /// x86-64 SSE4.1: roundss).
    #[inline]
    fn cell_i64(v: f64) -> i64 {
        (v * 0.25).floor() as i64
    }

    /// The (i64, i64) cell of a position (kept for diagnostics).
    #[inline]
    pub fn cell_of(x: f64, z: f64) -> (i64, i64) {
        (Self::cell_i64(x), Self::cell_i64(z))
    }

    /// Zigzag-pack a cell pair into one u64 (order-preserving, collision-
    /// free for the full i32 block range — the world border is ±30M).
    #[inline]
    fn cell_key_cc(cx: i64, cz: i64) -> u64 {
        (zigzag(cx) << 32) | zigzag(cz)
    }

    #[inline]
    fn cell_key(x: f64, z: f64) -> u64 {
        Self::cell_key_cc(Self::cell_i64(x), Self::cell_i64(z))
    }

    /// Full rebuild from a (slot, x, z) triples slice — three sequential
    /// passes (probe-count, prefix-sum over the compact occupied list,
    /// scatter). O(N) with zero per-tick table clearing: the generation
    /// stamp retires the whole previous tick in one integer add.
    pub fn rebuild(&mut self, tr: &[(u32, f64, f64)]) {
        // retire the previous tick's stamping
        self.cur_gen = self.cur_gen.wrapping_add(1);
        if self.cur_gen == 0 {
            self.gens.fill(0);
            self.cur_gen = 1;
        }
        // pre-size for the worst case (all cells unique, load <= 0.5):
        // growth happens before any stamping, so stale entries need no
        // rehash — just bigger arrays.
        let want = (tr.len() * 2).max(64).next_power_of_two();
        if want > self.keys.len() {
            self.keys = vec![0u64; want];
            self.gens = vec![0u32; want];
            self.rngs = vec![(0u32, 0u32); want];
            self.cursor = vec![0u32; want];
            self.mask = want - 1;
        }
        self.occ.clear();
        self.vals.clear();
        self.population = tr.len() as u64;
        // one exact multiply+floor per axis per mob
        self.kbuf.clear();
        for &(_, x, z) in tr {
            self.kbuf.push(Self::cell_key(x, z));
        }
        // pass 1: probe/insert, count per cell
        self.tmp_cell.clear();
        self.tmp_cell.resize(tr.len(), 0);
        for (j, &k) in self.kbuf.iter().enumerate() {
            let mut i = (cell_hash(k) as usize) & self.mask;
            loop {
                if self.gens[i] == self.cur_gen {
                    if self.keys[i] == k {
                        break; // found
                    }
                } else {
                    // claim an empty slot for the new generation
                    self.keys[i] = k;
                    self.gens[i] = self.cur_gen;
                    self.rngs[i] = (0, 0);
                    self.occ.push(i as u32);
                    break;
                }
                i = (i + 1) & self.mask;
            }
            self.tmp_cell[j] = i as u32;
            self.rngs[i].1 += 1;
        }
        // pass 2: prefix sums over the COMPACT occupied list — no table
        // walk, the cost is proportional to the unique cells, not capacity
        let mut run = 0u32;
        for &i in &self.occ {
            let iu = i as usize;
            let c = self.rngs[iu].1;
            self.rngs[iu] = (run, c);
            self.cursor[iu] = run;
            run += c;
        }
        self.vals.resize(run as usize, 0);
        // pass 3: scatter the slot ids into their cell ranges
        for (j, &(slot, _, _)) in tr.iter().enumerate() {
            let iu = self.tmp_cell[j] as usize;
            let p = self.cursor[iu];
            self.vals[p as usize] = slot;
            self.cursor[iu] = p + 1;
        }
    }

    /// Probe a packed cell: its (start, len) range when occupied this
    /// generation.
    #[inline]
    fn range_of(&self, k: u64) -> Option<(u32, u32)> {
        let mut i = (cell_hash(k) as usize) & self.mask;
        loop {
            if self.gens[i] != self.cur_gen {
                return None;
            }
            if self.keys[i] == k {
                return Some(self.rngs[i]);
            }
            i = (i + 1) & self.mask;
        }
    }

    /// A superset of the mobs whose position could touch the XZ area.
    /// Returned slot indices are unordered (Horizon contract: the caller
    /// re-tests exact boxes; order-sensitive callers sort by slot).
    pub fn candidates(
        &self,
        min_x: f64,
        min_z: f64,
        max_x: f64,
        max_z: f64,
        out: &mut Vec<u32>,
    ) {
        out.clear();
        let cx0 = Self::cell_i64(min_x);
        let cx1 = Self::cell_i64(max_x);
        let cz0 = Self::cell_i64(min_z);
        let cz1 = Self::cell_i64(max_z);
        // the overflow guard: an area spanning too many columns falls back
        // to a walk of the COMPACT occupied list (Horizon seen_grid.rs:69)
        let span_x = (cx1 - cx0).saturating_add(1) as usize;
        let span_z = (cz1 - cz0).saturating_add(1) as usize;
        if span_x.saturating_mul(span_z) > 1024 {
            for &i in &self.occ {
                let (s, l) = self.rngs[i as usize];
                out.extend_from_slice(&self.vals[s as usize..(s + l) as usize]);
            }
            return;
        }
        for cz in cz0..=cz1 {
            for cx in cx0..=cx1 {
                if let Some((s, l)) = self.range_of(Self::cell_key_cc(cx, cz)) {
                    out.extend_from_slice(&self.vals[s as usize..(s + l) as usize]);
                }
            }
        }
    }

    #[inline]
    pub fn cell_len(&self, cx: i64, cz: i64) -> usize {
        match self.range_of(Self::cell_key_cc(cx, cz)) {
            Some((_, l)) => l as usize,
            None => 0,
        }
    }

    pub fn clear(&mut self) {
        self.gens.fill(0);
        self.cur_gen = 1;
        self.occ.clear();
        self.vals.clear();
        self.population = 0;
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
                };
                s
            }
            None => {
                self.mobs.push(MobSlot {
                    mob,
                    alive: true,
                    grid_cell: (i64::MAX, i64::MAX),
                });
                (self.mobs.len() - 1) as u32
            }
        };
        self.index.insert(id, slot);
    }

    /// Remove by id (between ticks). Tombstones compact lazily: `compact`
    /// runs when tombstones exceed a quarter of the population.
    pub fn remove(&mut self, id: i64) {
        if let Some(slot) = self.index.remove(&id) {
            self.mobs[slot as usize].alive = false;
            self.free.push(slot);
        }
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
        m.goals = zombie_table();
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
        let tr: Vec<(u32, f64, f64)> = s
            .mobs
            .iter()
            .enumerate()
            .map(|(i, m)| (i as u32, m.mob.x, m.mob.z))
            .collect();
        g.rebuild(&tr);
        let mut out = Vec::new();
        g.candidates(-1.0, -1.0, 3.5, 1.0, &mut out);
        out.sort_unstable();
        assert_eq!(out, vec![0, 1]); // 4.1 is in the next cell — must NOT be missed by [0,3.5]
        // degenerate far-apart area: overflow fallback walks all cells
        g.candidates(-1.0e6, -1.0e6, 1.0e6, 1.0e6, &mut out);
        assert_eq!(out.len(), 3);
        // double-check the boundary: an area touching the boundary of a cell
        // must include the neighbour mob at 4.1 (superset, not exact)
        g.candidates(0.0, 0.0, 4.0, 0.0, &mut out);
        out.sort_unstable();
        assert!(out.contains(&2));
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

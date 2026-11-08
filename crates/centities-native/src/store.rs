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

/// The uniform 2-D XZ grid over mob slots. Cells are `HashMap<(i64,i64),
/// Vec<u32>)` — at 25k mobs in a pile the hot cells hold thousands of slots;
/// the candidates() contract is a SUPERSET (never misses a mob whose box
/// touches the area), which is all the floor's queries need (they re-test
/// exact boxes themselves).
#[derive(Default)]
pub struct SeenGrid {
    cells: HashMap<(i64, i64), Vec<u32>>,
    pub population: u64,
}

impl SeenGrid {
    #[inline]
    pub fn cell_of(x: f64, z: f64) -> (i64, i64) {
        (x.div_euclid(GRID_CELL) as i64, z.div_euclid(GRID_CELL) as i64)
    }

    /// Full rebuild — O(N) per tick, the Horizon shape (their update(i,b)
    /// incremental re-bucket is the N1 optimization; at 25k the rebuild is
    /// ~microseconds and the honest baseline for the profile).
    pub fn rebuild<'a, I: Iterator<Item = (u32, f64, f64)>>(&mut self, it: I) {
        self.cells.clear();
        self.population = 0;
        for (slot, x, z) in it {
            let c = Self::cell_of(x, z);
            self.cells.entry(c).or_default().push(slot);
            self.population += 1;
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
        let cx0 = min_x.div_euclid(GRID_CELL) as i64;
        let cx1 = max_x.div_euclid(GRID_CELL) as i64;
        let cz0 = min_z.div_euclid(GRID_CELL) as i64;
        let cz1 = max_z.div_euclid(GRID_CELL) as i64;
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
    pub fn cell_len(&self, cx: i64, cz: i64) -> usize {
        self.cells.get(&(cx, cz)).map_or(0, |c| c.len())
    }

    pub fn clear(&mut self) {
        self.cells.clear();
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
        g.rebuild(
            s.mobs
                .iter()
                .enumerate()
                .map(|(i, m)| (i as u32, m.mob.x, m.mob.z)),
        );
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

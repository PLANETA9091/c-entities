//! swarm-sim — the materialization engine between the far-field ledger
//! (swarm-core) and the exact per-entity index (centities-core).
//!
//! The 10^9 architecture (measured basis: scale_bench 0117187 — 117 B/ent
//! index, 1.34M upserts/s/core, push-query CPU wall):
//!
//! - T2 far field  = [SwarmLedger] exact per-chunk counts. No entities.
//! - T1 sim belt   = [EntityIndex] full exact entities inside the players'
//!   sim boundary. Vanilla-exact queries (the parity-proven path serves
//!   them from the module).
//! - T0 hot ring   = [HotRing] SoA storage for the per-tick hot loop near
//!   players (cap-bounded, swap_remove, zero allocation in steady state).
//!
//! A chunk crossing the sim boundary is a VIEW transition, not a fudge:
//! entities are materialized at their DETERMINISTIC slot positions
//! (chunk_slot_pos — the same chunk + slot always yields the same block),
//! and the world total `ledger.total + index.entities` is invariant across
//! any sequence of player moves (test-enforced). Outside the sim distance
//! vanilla Paper does not tick mobs either (ActivationRange/EAR) — a
//! skipped tick there is a provable zero-diff, so the ledger IS the exact
//! far-field state, not an approximation of it.

use centities_core::{Aabb, EntityIndex};
use std::collections::{HashMap, HashSet};
use swarm_core::{chunk_slot_pos, pack_chunk, ChunkDelta, SwarmError, SwarmLedger};

pub mod mob;

/// Zombie-ish live box for materialized entities (0.6 x 1.95).
const BOX_W: f64 = 0.6;
const BOX_H: f64 = 1.95;

/// Which chunks enter/leave the simulated set on a boundary update.
#[derive(Debug, Default, Clone)]
pub struct Transition {
    pub enter: Vec<u64>,
    pub leave: Vec<u64>,
}

/// The square (Chebyshev) sim boundary: the union of radius-R chunk
/// squares around every player chunk. Vanilla sim distance is per-player
/// and roughly circular; the square is the v0.1 over-approximation of the
/// simulated set — over-approximating the set of TICKED entities is
/// vanilla-conservative (ticking MORE entities than vanilla can only cost
/// CPU, never change behavior of the rest).
#[derive(Debug, Clone)]
pub struct SimBoundary {
    radius: i32,
    current: HashSet<u64>,
}

impl SimBoundary {
    pub fn new(radius: u32) -> Self {
        Self {
            radius: radius as i32,
            current: HashSet::new(),
        }
    }

    #[inline]
    pub fn contains(&self, key: u64) -> bool {
        self.current.contains(&key)
    }

    #[inline]
    pub fn materialized_chunks(&self) -> usize {
        self.current.len()
    }

    #[inline]
    pub fn radius(&self) -> i32 {
        self.radius
    }

    /// Recompute the boundary from player chunk positions; returns the
    /// enter/leave diff and latches the new set.
    pub fn update(&mut self, players: &[(i32, i32)]) -> Transition {
        let cells = (2 * self.radius + 1) as usize;
        let mut next: HashSet<u64> =
            HashSet::with_capacity(players.len().saturating_mul(cells * cells));
        for &(px, pz) in players {
            for dx in -self.radius..=self.radius {
                for dz in -self.radius..=self.radius {
                    next.insert(pack_chunk(px + dx, pz + dz));
                }
            }
        }
        let mut t = Transition::default();
        for &k in &next {
            if !self.current.contains(&k) {
                t.enter.push(k);
            }
        }
        for &k in &self.current {
            if !next.contains(&k) {
                t.leave.push(k);
            }
        }
        t.enter.sort_unstable();
        t.leave.sort_unstable();
        self.current = next;
        t
    }
}

/// The T0 hot ring: cap-bounded SoA storage for per-tick hot entities.
///
/// Storage only — the tick logic is the parity-proven engine's business.
/// What the ring guarantees:
/// - admission never allocates after construction (cap-preallocated SoA +
///   id map);
/// - eviction is swap_remove — O(1), the last entity takes the freed slot
///   and its id→slot mapping is fixed up (id stability under eviction
///   storms is test-enforced);
/// - the ring NEVER exceeds `cap` — the hot loop's worst case is the cap,
///   not the world.
#[derive(Debug)]
pub struct HotRing {
    cap: usize,
    id: Vec<i64>,
    px: Vec<f32>,
    py: Vec<f32>,
    pz: Vec<f32>,
    vx: Vec<f32>,
    vz: Vec<f32>,
    hp: Vec<i16>,
    ai_timer: Vec<u8>,
    state: Vec<u8>,
    home_chunk: Vec<u64>,
    slot: Vec<u32>,
    pos_of: HashMap<i64, usize>,
}

impl HotRing {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            id: Vec::with_capacity(cap),
            px: Vec::with_capacity(cap),
            py: Vec::with_capacity(cap),
            pz: Vec::with_capacity(cap),
            vx: Vec::with_capacity(cap),
            vz: Vec::with_capacity(cap),
            hp: Vec::with_capacity(cap),
            ai_timer: Vec::with_capacity(cap),
            state: Vec::with_capacity(cap),
            home_chunk: Vec::with_capacity(cap),
            slot: Vec::with_capacity(cap),
            pos_of: HashMap::with_capacity(cap),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.id.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id.is_empty()
    }

    #[inline]
    pub fn cap(&self) -> usize {
        self.cap
    }

    /// O(1) admission; `false` when full (the caller evicts by policy and
    /// retries — the ring itself never decides WHO is hot).
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn admit(
        &mut self,
        id: i64,
        home_chunk: u64,
        slot: u32,
        px: f32,
        py: f32,
        pz: f32,
        vx: f32,
        vz: f32,
        hp: i16,
        ai_timer: u8,
        state: u8,
    ) -> bool {
        if self.id.len() >= self.cap || self.pos_of.contains_key(&id) {
            return false;
        }
        let i = self.id.len();
        self.id.push(id);
        self.home_chunk.push(home_chunk);
        self.slot.push(slot);
        self.px.push(px);
        self.py.push(py);
        self.pz.push(pz);
        self.vx.push(vx);
        self.vz.push(vz);
        self.hp.push(hp);
        self.ai_timer.push(ai_timer);
        self.state.push(state);
        self.pos_of.insert(id, i);
        true
    }

    /// O(1) eviction by id (swap_remove + id-map fixup).
    pub fn evict(&mut self, id: i64) -> bool {
        let Some(&i) = self.pos_of.get(&id) else {
            return false;
        };
        let last = self.id.len() - 1;
        let moved = self.id[last];
        // self-swap is a no-op for the map but MUST not reinsert twice
        if i != last {
            self.id[i] = moved;
            self.px[i] = self.px[last];
            self.py[i] = self.py[last];
            self.pz[i] = self.pz[last];
            self.vx[i] = self.vx[last];
            self.vz[i] = self.vz[last];
            self.hp[i] = self.hp[last];
            self.ai_timer[i] = self.ai_timer[last];
            self.state[i] = self.state[last];
            self.home_chunk[i] = self.home_chunk[last];
            self.slot[i] = self.slot[last];
            self.pos_of.insert(moved, i);
        }
        self.id.truncate(last);
        self.px.truncate(last);
        self.py.truncate(last);
        self.pz.truncate(last);
        self.vx.truncate(last);
        self.vz.truncate(last);
        self.hp.truncate(last);
        self.ai_timer.truncate(last);
        self.state.truncate(last);
        self.home_chunk.truncate(last);
        self.slot.truncate(last);
        self.pos_of.remove(&id);
        true
    }

    #[inline]
    pub fn contains(&self, id: i64) -> bool {
        self.pos_of.contains_key(&id)
    }

    #[inline]
    pub fn slot_of(&self, id: i64) -> Option<usize> {
        self.pos_of.get(&id).copied()
    }

    #[inline]
    pub fn home_of(&self, i: usize) -> (u64, u32) {
        (self.home_chunk[i], self.slot[i])
    }

    /// SoA field access for the hot loop (slot comes from `slot_of`).
    #[inline]
    pub fn pos(&self, i: usize) -> (f32, f32, f32) {
        (self.px[i], self.py[i], self.pz[i])
    }

    #[inline]
    pub fn set_pos(&mut self, i: usize, x: f32, y: f32, z: f32) {
        self.px[i] = x;
        self.py[i] = y;
        self.pz[i] = z;
    }

    #[inline]
    pub fn vel(&self, i: usize) -> (f32, f32) {
        (self.vx[i], self.vz[i])
    }

    #[inline]
    pub fn set_vel(&mut self, i: usize, vx: f32, vz: f32) {
        self.vx[i] = vx;
        self.vz[i] = vz;
    }

    #[inline]
    pub fn hp(&self, i: usize) -> i16 {
        self.hp[i]
    }

    #[inline]
    pub fn set_hp(&mut self, i: usize, hp: i16) {
        self.hp[i] = hp;
    }

    #[inline]
    pub fn state(&self, i: usize) -> (u8, u8) {
        (self.ai_timer[i], self.state[i])
    }

    #[inline]
    pub fn set_state(&mut self, i: usize, ai_timer: u8, state: u8) {
        self.ai_timer[i] = ai_timer;
        self.state[i] = state;
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = i64> + '_ {
        self.id.iter().copied()
    }
}

/// The full swarm engine: far-field ledger + exact index + boundary driver.
///
/// World-total contract: `ledger.total + index.entities` is INVARIANT
/// across any sequence of `set_players` calls (materialization moves units
/// between the two halves; it never creates or destroys them).
pub struct SwarmEngine {
    pub ledger: SwarmLedger,
    pub index: EntityIndex,
    pub world: u16,
    boundary: SimBoundary,
    /// materialized entity ids per chunk (dematerialization bookkeeping)
    ids_by_chunk: HashMap<u64, Vec<i64>>,
    materialized: u64,
    next_id: i64,
}

/// Per-boundary-update accounting (the tick driver's budget line).
#[derive(Debug, Default, Clone, Copy)]
pub struct TransitionCost {
    pub chunks_in: usize,
    pub chunks_out: usize,
    pub materialized: u64,
    pub dematerialized: u64,
}

impl SwarmEngine {
    pub fn new(world: u16, radius: u32) -> Self {
        Self {
            ledger: SwarmLedger::new(),
            index: EntityIndex::new(),
            world,
            boundary: SimBoundary::new(radius),
            ids_by_chunk: HashMap::new(),
            materialized: 0,
            next_id: 1,
        }
    }

    /// Seed the far field (must be called for chunks OUTSIDE the current
    /// boundary; seeding a simulated chunk would double-account it).
    pub fn seed_far(&mut self, batch: &[ChunkDelta]) -> Result<(), SwarmError> {
        for d in batch {
            debug_assert!(
                !self.boundary.contains(d.key),
                "seeding a materialized chunk {d:?}"
            );
        }
        self.ledger.apply(batch)
    }

    #[inline]
    pub fn world_total(&self) -> u64 {
        self.ledger.total() + self.materialized
    }

    #[inline]
    pub fn materialized(&self) -> u64 {
        self.materialized
    }

    #[inline]
    pub fn boundary(&self) -> &SimBoundary {
        &self.boundary
    }

    /// Deterministic entity box for (chunk, slot).
    #[inline]
    fn slot_aabb(key: u64, slot: u32) -> Aabb {
        let (x, y, z) = chunk_slot_pos(key, slot);
        Aabb::new(
            x as f64,
            y as f64,
            z as f64,
            x as f64 + BOX_W,
            y as f64 + BOX_H,
            z as f64 + BOX_H,
        )
    }

    /// Move the player set; materialize/dematerialize the diff.
    ///
    /// Atomicity: the ledger batch (leave→+n, enter→−n) is validated and
    /// committed ONCE before any index mutation; index ops are infallible,
    /// so a rejected transition leaves BOTH halves untouched.
    pub fn set_players(&mut self, players: &[(i32, i32)]) -> Result<TransitionCost, SwarmError> {
        let t = self.boundary.update(players);
        if t.enter.is_empty() && t.leave.is_empty() {
            return Ok(TransitionCost::default());
        }
        // build the single atomic ledger batch; capture the enter counts
        // BEFORE the apply (after it the ledger count is 0 — the units moved
        // to the index side)
        let mut batch: Vec<ChunkDelta> = Vec::with_capacity(t.enter.len() + t.leave.len());
        for &k in &t.leave {
            // n == 0 is LEGAL: the chunk may have entered with a zero
            // ledger count (an empty chunk inside the boundary). Its
            // leave-delta is then 0 — a ledger no-op.
            let n = self.ids_by_chunk.get(&k).map(|v| v.len()).unwrap_or(0) as i32;
            batch.push(ChunkDelta { key: k, delta: n });
        }
        let mut enter_counts: Vec<(u64, u32)> = Vec::with_capacity(t.enter.len());
        for &k in &t.enter {
            let n = self.ledger.chunk_count(k);
            enter_counts.push((k, n));
            batch.push(ChunkDelta {
                key: k,
                delta: -(n as i32),
            });
        }
        self.ledger.apply(&batch)?;

        // index side (infallible)
        let mut cost = TransitionCost {
            chunks_in: t.enter.len(),
            chunks_out: t.leave.len(),
            ..Default::default()
        };
        for &k in &t.leave {
            if let Some(ids) = self.ids_by_chunk.remove(&k) {
                let n = ids.len() as u64;
                for id in &ids {
                    self.index.remove(*id);
                }
                cost.dematerialized += n;
                self.materialized -= n;
            }
        }
        for &(k, n) in &enter_counts {
            let mut ids = Vec::new();
            for slot in 0..n {
                let id = self.next_id;
                self.next_id += 1;
                let b = Self::slot_aabb(k, slot);
                self.index.upsert(id, self.world, &b);
                ids.push(id);
            }
            cost.materialized += n as u64;
            self.materialized += n as u64;
            self.ids_by_chunk.insert(k, ids);
        }
        Ok(cost)
    }

    /// Hot-ring admission for ONE player's neighborhood: admit every
    /// materialized entity in the hot square (hot_radius < sim radius),
    /// cap-enforced. Returns the number admitted.
    pub fn fill_ring(&mut self, ring: &mut HotRing, pchunk: (i32, i32), hot_radius: i32) -> usize {
        let mut admitted = 0;
        for dx in -hot_radius..=hot_radius {
            for dz in -hot_radius..=hot_radius {
                let key = pack_chunk(pchunk.0 + dx, pchunk.1 + dz);
                let Some(ids) = self.ids_by_chunk.get(&key) else {
                    continue;
                };
                for (slot, &id) in ids.iter().enumerate() {
                    if ring.len() >= ring.cap() {
                        return admitted;
                    }
                    if ring.contains(id) {
                        continue;
                    }
                    let (x, y, z) = chunk_slot_pos(key, slot as u32);
                    if ring.admit(
                        id,
                        key,
                        slot as u32,
                        x as f32,
                        y as f32,
                        z as f32,
                        0.0,
                        0.0,
                        20,
                        0,
                        0,
                    ) {
                        admitted += 1;
                    }
                }
            }
        }
        admitted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rng_next(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ z >> 31
    }

    #[test]
    fn world_total_invariant_under_player_walks() {
        let mut eng = SwarmEngine::new(0, 8);
        let mut rng = 0xFEEDu64;
        // seed a 64x64-chunk world region, 0..64 entities per chunk
        let mut batch = Vec::new();
        for cx in -32..32 {
            for cz in -32..32 {
                let n = (rng_next(&mut rng) % 64) as i32;
                if n > 0 {
                    batch.push(ChunkDelta::new(cx, cz, n));
                }
            }
        }
        eng.seed_far(&batch).unwrap();
        let total = eng.world_total();
        assert!(total > 100_000);

        // 200 random player teleports; the total must NEVER move
        for step in 0..200 {
            let px = (rng_next(&mut rng) % 128) as i32 - 64;
            let pz = (rng_next(&mut rng) % 128) as i32 - 64;
            eng.set_players(&[(px, pz)]).unwrap();
            assert_eq!(
                eng.world_total(),
                total,
                "world total drifted at step {step} (players at {px},{pz})"
            );
            // index and internal counter agree
            assert_eq!(eng.index.stats().entities, eng.materialized as usize);
        }
    }

    #[test]
    fn boundary_diff_is_exact_and_idempotent() {
        let mut b = SimBoundary::new(2);
        let t = b.update(&[(0, 0)]);
        assert_eq!(t.enter.len(), 25); // 5x5
        assert!(t.leave.is_empty());
        // same spot again → no churn
        let t = b.update(&[(0, 0)]);
        assert!(t.enter.is_empty() && t.leave.is_empty());
        // move by 1 → one column enters, one leaves
        let t = b.update(&[(1, 0)]);
        assert_eq!(t.enter.len(), 5);
        assert_eq!(t.leave.len(), 5);
        // two players overlapping → union, no double-count:
        // 9x9 at (0,0) ∪ 9x9 at (1,0): overlap 8x9=72, union = 81+81-72 = 90
        let mut b2 = SimBoundary::new(4);
        let t = b2.update(&[(0, 0), (1, 0)]);
        assert_eq!(t.enter.len(), 90);
    }

    #[test]
    fn materialization_is_deterministic() {
        let mut eng = SwarmEngine::new(0, 1);
        eng.seed_far(&[ChunkDelta::new(10, 10, 100)]).unwrap();
        eng.set_players(&[(10, 10)]).unwrap();
        let key = pack_chunk(10, 10);
        let ids1: Vec<i64> = eng.ids_by_chunk[&key].clone();
        let boxes1: Vec<(i32, i32, i32)> = (0..ids1.len() as u32)
            .map(|slot| chunk_slot_pos(key, slot))
            .collect();
        // dematerialize and rematerialize the same chunk
        eng.set_players(&[(50, 50)]).unwrap();
        assert_eq!(eng.materialized(), 0);
        eng.set_players(&[(10, 10)]).unwrap();
        let ids2: Vec<i64> = eng.ids_by_chunk[&key].clone();
        let boxes2: Vec<(i32, i32, i32)> = (0..ids2.len() as u32)
            .map(|slot| chunk_slot_pos(key, slot))
            .collect();
        assert_eq!(
            boxes1, boxes2,
            "slot positions must be deterministic across rematerialization"
        );
        assert_eq!(ids1.len(), ids2.len());
        // and the index actually holds them at those boxes
        for (slot, &id) in ids2.iter().enumerate() {
            let (x, y, z) = chunk_slot_pos(key, slot as u32);
            let b = Aabb::new(
                x as f64,
                y as f64,
                z as f64,
                x as f64 + BOX_W,
                y as f64 + BOX_H,
                z as f64 + BOX_H,
            );
            let mut out = Vec::new();
            eng.index.query(0, &b, &mut out, centities_core::TICK_UNKNOWN);
            assert!(
                out.contains(&id),
                "materialized id {id} not queryable at its slot box"
            );
        }
    }

    #[test]
    fn hot_ring_survives_eviction_storms() {
        let mut ring = HotRing::new(64);
        // fill to cap
        for i in 1..=64i64 {
            assert!(ring.admit(i, 0, i as u32, i as f32, 64.0, i as f32, 0.0, 0.0, 20, 0, 0));
        }
        assert!(
            !ring.admit(65, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0, 0),
            "cap enforced"
        );
        // eviction storm: remove every odd id, then re-admit new ones
        for i in (1..=64i64).step_by(2) {
            assert!(ring.evict(i));
        }
        assert_eq!(ring.len(), 32);
        for i in 100..132i64 {
            assert!(ring.admit(i, 0, i as u32, i as f32, 64.0, i as f32, 0.0, 0.0, 20, 0, 0));
        }
        assert_eq!(ring.len(), 64);
        // map consistency: every surviving id resolves, every slot unique
        let mut seen = [false; 64];
        for id in ring.iter_ids() {
            let s = ring.slot_of(id).expect("id map broken");
            assert!(!seen[s], "two ids share slot {s}");
            seen[s] = true;
            let (px, py, pz) = ring.pos(s);
            assert!(px.is_finite() && py.is_finite() && pz.is_finite());
        }
        assert!(seen.iter().all(|&s| s), "stale slots after storm");
        assert_eq!(ring.pos_of.len(), ring.len(), "no duplicate id map entries");
        // evicting an unknown id is a clean false
        assert!(!ring.evict(999_999));
    }

    #[test]
    fn fill_ring_admits_nearest_chunks_and_caps() {
        let mut eng = SwarmEngine::new(0, 4);
        // 9x9 chunks around origin, 10 entities per chunk
        let mut batch = Vec::new();
        for cx in -4..=4 {
            for cz in -4..=4 {
                batch.push(ChunkDelta::new(cx, cz, 10));
            }
        }
        eng.seed_far(&batch).unwrap();
        eng.set_players(&[(0, 0)]).unwrap();
        assert_eq!(eng.materialized(), 810);

        let mut ring = HotRing::new(100);
        let n = eng.fill_ring(&mut ring, (0, 0), 1); // 3x3 chunks = 90 entities
        assert_eq!(n, 90);
        assert_eq!(ring.len(), 90);
        // everything in the ring must be from the hot 3x3 square
        for id in ring.iter_ids() {
            let s = ring.slot_of(id).unwrap();
            let (hc, _slot) = ring.home_of(s);
            let (cx, cz) = swarm_core::unpack_chunk(hc);
            assert!(cx.abs() <= 1 && cz.abs() <= 1);
        }
        // cap: shrink the ring to 50 → refill must stop at 50
        let mut small = HotRing::new(50);
        let n = eng.fill_ring(&mut small, (0, 0), 1);
        assert_eq!(n, 50);
        assert_eq!(small.len(), 50);
    }

    #[test]
    fn ledger_index_split_survives_churn() {
        // walk one player 500 steps in a small world; after every step:
        // - ledger.total + materialized == seeded total
        // - every materialized chunk's id list length == its former count
        // - ledger never holds a materialized chunk (no double accounting)
        let mut eng = SwarmEngine::new(0, 2);
        let mut rng = 0xBEEFu64;
        let mut batch = Vec::new();
        for cx in -16..16 {
            for cz in -16..16 {
                batch.push(ChunkDelta::new(
                    cx,
                    cz,
                    ((rng_next(&mut rng) % 40) + 1) as i32,
                ));
            }
        }
        eng.seed_far(&batch).unwrap();
        let total = eng.world_total();
        let mut px = 0i32;
        let mut pz = 0i32;
        for _ in 0..500 {
            px += (rng_next(&mut rng) % 3) as i32 - 1;
            pz += (rng_next(&mut rng) % 3) as i32 - 1;
            eng.set_players(&[(px, pz)]).unwrap();
            assert_eq!(eng.world_total(), total);
            for &k in eng.ids_by_chunk.keys() {
                assert_eq!(eng.ledger.chunk_count(k), 0, "chunk {k} double-accounted");
            }
        }
    }
}

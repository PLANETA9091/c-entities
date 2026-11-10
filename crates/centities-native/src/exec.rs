//! The native full-tick driver: ONE call ticks the whole managed
//! population — AI floor → travel slice → grid refresh → compaction. This
//! is the function the Java host will call once per tick through JNI (the
//! L111 bridge); in Rust it is a plain function call, so the profiler sees
//! EVERY frame.
//!
//! Tick shape (the Horizon contract, creatures.rs:1251): vanilla insertion
//! order, one mob at a time, AI before movement, later mobs observe earlier
//! mobs' results. The profile phases (ai/travel/grid/apply) split the same
//! body the way Horizon's spans do.
use swarm_sim::mob::{AiStat, MobAI, PackScratch, PlayerSnap};
use swarm_sim::mob::TravelMth as Mth;

use centities_core::goals as g;

use crate::profile::{ai_guard, apply_guard, grid_guard, travel_guard};
use crate::store::{MobStore, SeenGrid};
use crate::world::{PlayerTick, WorldRes};

/// The executor state: the persistent store + the grid + the shared travel
/// context (`Mth` — the sine/cosine table, one instance for the world).
pub struct NativeTick {
    pub store: MobStore,
    pub grid: SeenGrid,
    pub mth: Mth,
    /// tombstone compaction threshold (free slots) — the Horizon lazy
    /// compact (their slot lists rebuild when tombstones pile up)
    pub compact_at: usize,
    /// the splitmix stroll stream (owned here, handed to the resolver per
    /// mob — the stream advances across mobs within the tick, vanilla
    /// draws it from the mob's OWN random; the swarm-honesty note 1
    /// split stays until the L111 bridge replaces it with the real
    /// per-mob stream handoff)
    pub target_rng: u64,
    /// reused (slot, x, z) triples for the grid rebuild (zero steady-
    /// state allocation — the L111 line)
    triples: Vec<(u32, f64, f64)>,
    /// the executor's ONE shared aiPack wire scratch (L111c + the L112c
    /// static stamp): ~456B in L1 across the whole AI loop
    ai_scratch: PackScratch,
    /// the executor's ONE shared floor OUT buffer (L112): the old shape
    /// zeroed an 808-word stack array PER MOB PER TICK (25k mobs x 808
    /// words = 20M dead stores per tick). One persistent buffer serves
    /// the whole loop: the floor writes every slot the executor reads
    /// before the read, so stale words beyond the current action count
    /// are never observed.
    out_scratch: Vec<i32>,
    /// bumped every tick; the grid rebuild LAZY stamp — the grid is
    /// rebuilt on the FIRST query after the bump, never eagerly (L111e:
    /// nothing in the v0 executor queries candidates — the AI's player
    /// scans walk the snapshot O(P) — so the per-tick rebuild was pure
    /// dead work; the future push kernel pays for the build at its first
    /// query of the tick)
    tick_epoch: u32,
    /// the epoch the grid was last rebuilt for (0 = never built)
    grid_built_epoch: u32,
    /// the per-tick AI accounting, reused across ticks (the old
    /// `Vec::with_capacity` + return allocated ~800KB per tick — mmap
    /// + 200 first-touch page faults EVERY tick, pure overhead: the
    /// bench and the tests discard the value)
    stats_scratch: Vec<AiStat>,
}

impl NativeTick {
    pub fn new(target_seed: u64) -> Self {
        NativeTick {
            store: MobStore::new(),
            grid: SeenGrid::default(),
            mth: Mth::new(),
            compact_at: 256,
            target_rng: target_seed,
            triples: Vec::new(),
            ai_scratch: PackScratch::new(),
            out_scratch: vec![0i32; g::OUT_MIN_LEN],
            tick_epoch: 0,
            grid_built_epoch: 0,
            stats_scratch: Vec::new(),
        }
    }

    /// Host push: replace the player snapshot (the COPY layer contract —
    /// the host serializes nothing else; players only, mobs live in the
    /// store permanently).
    pub fn push_players(&mut self, players: Vec<PlayerTick>) {
        self.store.snapshot.players = players;
    }

    /// Host push: one mob's persistent state created (spawn / handoff from
    /// Java at load-in). The caller supplies the full `MobAI` (the bridge
    /// packs the goal table, RNG mirror and physics fields verbatim).
    pub fn add_mob(&mut self, mob: MobAI) {
        self.store.add(mob);
    }

    pub fn remove_mob(&mut self, id: i64) {
        self.store.remove(id);
    }

    /// The whole managed tick. Returns the per-mob AI accounting (a slice
    /// into the executor's reused buffer — valid until the next tick).
    pub fn tick(&mut self, game_time: i32) -> &[AiStat] {
        // the grid is LAZY now (L111e): no per-tick rebuild. The executor
        // bumps the epoch and the first query of the tick pays for the
        // build. The v0 executor's AI never queries candidates (player
        // scans walk the snapshot), so the steady-state grid cost is zero.
        crate::profile::tick_open();
        self.tick_epoch = self.tick_epoch.wrapping_add(1);
        // players -> the pack snapshot (the floor borrows it per mob;
        // built once per tick, P is small)
        let players: Vec<PlayerSnap> = {
            let _g = apply_guard();
            self.store
                .snapshot
                .players
                .iter()
                .map(|p: &PlayerTick| PlayerSnap {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                    id: p.id,
                    flags: p.flags,
                })
                .collect()
        };
        // the melee approach refresh: the floor reads approach_x/z live —
        // they must track the target's CURRENT position (vanilla reads
        // getTarget().position() per round through the pack). One pass
        // over target-holding mobs (no aliasing: mobs and snapshot are
        // separate fields).
        {
            let _g = apply_guard();
            for slot in self.store.mobs.iter_mut() {
                let m = &slot.mob;
                if m.target_player >= 0 {
                    let tp = m.target_player as usize;
                    if let Some(p) = self.store.snapshot.players.get(tp) {
                        slot.mob.approach_x = p.x;
                        slot.mob.approach_z = p.z;
                    } else {
                        slot.mob.target_player = -1;
                    }
                }
            }
        }
        // THE LOOP — vanilla order (insertion), ONE mob at a time, AI
        // before movement INSIDE the same iteration (L111f: the old shape
        // ran two passes over the ~11MB mob array — every mob's cache
        // lines were touched twice per tick, the second pass cold; the
        // vanilla body ticks each entity's AI and travel together, so
        // the merged shape is ALSO the more vanilla-exact one: a later
        // mob's AI now observes the earlier mobs' POST-TRAVEL positions,
        // the Horizon creatures.rs:1468 contract, not just their
        // post-AI/pre-travel ones). The resolver borrows the snapshot
        // (immutable) while the mob mutates — disjoint fields, the
        // borrow splits cleanly.
        let mut stats = std::mem::take(&mut self.stats_scratch);
        stats.clear();
        let snapshot: &crate::world::Snapshot = &self.store.snapshot;
        let scratch = &mut self.ai_scratch;
        let out_scratch = &mut self.out_scratch;
        let mth = &self.mth;
        for slot in self.store.mobs.iter_mut() {
            if !slot.alive {
                continue;
            }
            let stat = {
                let _g = ai_guard();
                let mut res = WorldRes::open_ground_v0(snapshot, 0);
                res.target_rng = self.target_rng;
                let st = slot.mob.ai_tick(&players, game_time, &mut res, scratch, out_scratch);
                self.target_rng = res.target_rng;
                st
            };
            {
                let _g = travel_guard();
                slot.mob.move_tick(mth);
            }
            stats.push(stat);
        }
        self.stats_scratch = stats;
        // tombstone compaction (between ticks, the Horizon lazy compact)
        if self.store.free.len() > self.compact_at {
            self.store.compact();
        }
        self.store.game_time = game_time;
        &self.stats_scratch
    }

    /// Lazy grid build: rebuilds the grid for the CURRENT tick epoch on
    /// the first query (the epoch stamp makes the check one integer
    /// compare). Consumers call the query helpers below; nothing here
    /// runs unless a consumer exists.
    fn ensure_grid(&mut self) {
        if self.grid_built_epoch == self.tick_epoch {
            return;
        }
        let _g = grid_guard();
        self.triples.clear();
        for (i, m) in self.store.mobs.iter().enumerate() {
            if m.alive {
                self.triples.push((i as u32, m.mob.x, m.mob.z));
            }
        }
        let tr = &self.triples;
        self.grid.rebuild(tr);
        self.grid_built_epoch = self.tick_epoch;
    }

    /// The candidates superset for the XZ area (rebuilds lazily on the
    /// first query of the tick). The push kernel's entry point.
    pub fn grid_candidates(
        &mut self,
        min_x: f64,
        min_z: f64,
        max_x: f64,
        max_z: f64,
        out: &mut Vec<u32>,
    ) {
        self.ensure_grid();
        self.grid.candidates(min_x, min_z, max_x, max_z, out);
    }

    /// Mob count in one grid cell (rebuilds lazily on the first query of
    /// the tick).
    pub fn grid_cell_len(&mut self, cx: i64, cz: i64) -> usize {
        self.ensure_grid();
        self.grid.cell_len(cx, cz)
    }
}

impl Default for NativeTick {
    fn default() -> Self {
        Self::new(0x5EED_1A11_0B1C)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swarm_sim::mob::zombie_table;
    use swarm_sim::mob::MobAI;

    fn mk(id: i64, x: f64, z: f64) -> MobAI {
        let mut m = MobAI::new(id, 0x1234_5678_9abc_def0 + id as u64, x, 64.0, z);
        m.goals = zombie_table();
        m
    }

    #[test]
    fn ticks_run_and_move_without_players() {
        let mut nt = NativeTick::new(42);
        for i in 0..64 {
            nt.add_mob(mk(i, i as f64 * 3.0, (i % 8) as f64 * 5.0));
        }
        let mut any_walked = false;
        for t in 0..200 {
            nt.tick(t);
            if t == 199 {
                for m in &nt.store.mobs {
                    if (m.mob.x - (m.mob.id % 64) as f64 * 3.0).abs() > 1e-6
                        || m.mob.nav_state != 0
                        || m.mob.goals.iter().any(|g| g.running)
                    {
                        any_walked = true;
                    }
                }
            }
        }
        assert!(any_walked, "peaceful mobs must stroll/look around");
        assert_eq!(nt.store.len(), 64);
    }

    #[test]
    fn target_acquisition_and_chase() {
        let mut nt = NativeTick::new(1337);
        for i in 0..32 {
            nt.add_mob(mk(i, i as f64 * 2.0 + 100.0, 100.0));
        }
        // one player inside follow range of several mobs
        nt.push_players(vec![PlayerTick {
            x: 100.0,
            y: 64.0,
            z: 96.0,
            id: 1,
            flags: 1, // PF_ALIVE
        }]);
        for t in 0..400 {
            nt.tick(t);
        }
        let chasing = nt
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "zombies must acquire the target: got {chasing}");
        // at least one mob materially closed distance toward the player
        let mut closed = false;
        for m in &nt.store.mobs {
            if m.mob.target_player >= 0 {
                let dx = m.mob.x - 100.0;
                let dz = m.mob.z - 100.0;
                if dx * dx + dz * dz < 900.0 {
                    closed = true;
                }
            }
        }
        assert!(closed, "a chasing mob must approach the player");
    }

    #[test]
    fn determinism_same_seed_same_endgame() {
        let run = |seed: u64| -> Vec<(u64, u64)> {
            let mut nt = NativeTick::new(seed);
            for i in 0..32 {
                nt.add_mob(mk(i, i as f64, 0.0));
            }
            for t in 0..120 {
                nt.tick(t);
            }
            nt.store
                .mobs
                .iter()
                .map(|m| (m.mob.x.to_bits(), m.mob.z.to_bits()))
                .collect()
        };
        assert_eq!(run(777), run(777), "same seed -> bitwise same positions");
        assert_ne!(run(777), run(778), "different stroll stream -> different path");
    }

    #[test]
    fn grid_rebuild_is_lazy_and_query_fresh() {
        let mut nt = NativeTick::new(9);
        for i in 0..8 {
            nt.add_mob(mk(i, i as f64 * 5.0, 0.0));
        }
        nt.tick(1);
        // no consumer queried -> the grid was NEVER built this tick
        assert_eq!(nt.grid.population, 0, "lazy grid must not build eagerly");
        // the first query builds for the current tick and answers exactly
        let mut out = Vec::new();
        nt.grid_candidates(-1.0, -1.0, 100.0, 1.0, &mut out);
        out.sort_unstable();
        assert_eq!(out, (0..8).collect::<Vec<u32>>());
        // a mob moves; the NEXT tick's query must see the fresh position
        // (the epoch stamp invalidates the previous build)
        nt.store.mobs[0].mob.x = 500.0;
        nt.tick(2);
        nt.grid_candidates(495.0, -1.0, 505.0, 1.0, &mut out);
        assert!(out.contains(&0), "moved mob must be found at its new cell");
        assert_eq!(nt.grid_cell_len(125, 0), 1);
    }

    #[test]
    fn remove_and_readd_keeps_population_exact() {
        let mut nt = NativeTick::new(7);
        for i in 0..100 {
            nt.add_mob(mk(i, i as f64, 0.0));
        }
        for i in (0..100).step_by(2) {
            nt.remove_mob(i);
        }
        assert_eq!(nt.store.len(), 50);
        nt.tick(1);
        for i in (0..100).step_by(2) {
            nt.remove_mob(i); // double remove is a no-op
        }
        assert_eq!(nt.store.len(), 50);
        for i in (0..100).step_by(2) {
            nt.add_mob(mk(1000 + i, i as f64, 0.0));
        }
        assert_eq!(nt.store.len(), 100);
        nt.store.compact();
        assert_eq!(nt.store.len(), 100);
        assert_eq!(nt.store.mobs.len(), 100);
    }
}

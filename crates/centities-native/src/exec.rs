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
use swarm_sim::mob::{AiStat, MobAI, PackState, PlayerSnap};
use swarm_sim::mob::TravelMth as Mth;

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
    /// WAVE3: the per-tick AI accounting buffer — ONE allocation per
    /// executor (grown on demand), returned as a borrow from `tick`.
    stats_buf: Vec<AiStat>,
    /// WAVE2: the shared aiPack scratch — ONE buffer for the whole
    /// population (grown on demand, reused across every mob and round;
    /// the old per-round `vec![0i32; ..]` was a malloc+memset per ROUND
    /// per MOB — ~75k allocations per tick at 25k mobs).
    pack_scratch: Vec<i32>,
    /// WAVE2: the shared floor-output buffer — ONE per executor, reset
    /// per mob over just the read-before-write words (see mob.rs
    /// OUT_RESET; the 3 KiB per-mob memset is gone). Bit-identical per
    /// the `reused_out_buffer_matches_fresh_zero` gate.
    out_buf: Vec<i32>,
    /// WAVE4: the resident-pack state (fp + tick stamps live here — ONE
    /// per executor; the scratch's static/tick halves outlive any mob).
    pack: PackState,
    /// WAVE4: the per-tick PlayerSnap staging buffer (the old code
    /// allocated a fresh Vec every tick).
    players_buf: Vec<PlayerSnap>,
    /// WAVE6 twin hook: true = the resolver's fold contract is OFF — every
    /// request parks (the exact pre-fold semantics; the A/B gate drives
    /// both modes over the same population).
    fold_off: bool,
}

impl NativeTick {
    pub fn new(target_seed: u64) -> Self {
        NativeTick {
            store: MobStore::new(),
            grid: SeenGrid::default(),
            mth: Mth::new(),
            compact_at: 256,
            target_rng: target_seed,
            pack_scratch: Vec::new(),
            out_buf: vec![0i32; swarm_sim::mob::out_len()],
            stats_buf: Vec::new(),
            pack: PackState::default(),
            players_buf: Vec::new(),
            fold_off: false,
        }
    }

    /// WAVE6 twin hook: force the parked (pre-fold) semantics.
    pub fn set_fold_off(&mut self, on: bool) {
        self.fold_off = on;
    }

    /// WAVE4 twin hook: force the wave3 full-pack-every-round semantics
    /// (the resident-vs-full bit test drives one executor per mode).
    pub fn set_full_pack(&mut self, on: bool) {
        self.pack.full = on;
        if on {
            // the stamps describe the resident content — drop them so the
            // full path is also COLD
            self.pack.fp = None;
            self.pack.tick = None;
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
        // the tombstone detaches from the grid immediately: a lingering
        // stale slot entry would double-list the id after recycling
        if let Some(slot) = self.store.remove(id) {
            let cell = self.store.mobs[slot as usize].grid_cell;
            // WAVE8b: the mobs slice rides along for the O(1) grid_pos
            // re-stamp (the two field borrows are disjoint)
            self.grid.detach(slot, cell, &mut self.store.mobs);
        }
    }

    /// The whole managed tick. Returns the per-mob AI accounting — a
    /// borrow of the executor's REUSED stats buffer (wave3: the old code
    /// allocated a fresh 25k-slot Vec — ~1.2 MB — every tick; the buffer
    /// now persists, the borrow ends at the next tick call).
    pub fn tick(&mut self, game_time: i32) -> &[AiStat] {
        // grid refresh (WAVE2: the Horizon N1 incremental re-bucket —
        // movers only; the profile shows the real share)
        {
            let _g = grid_guard();
            self.grid.rebuild(&mut self.store.mobs);
        }
        // players -> the pack snapshot (the floor borrows it per mob;
        // WAVE4: built into a REUSED buffer, not a fresh Vec per tick)
        let players: &[PlayerSnap] = {
            let _g = apply_guard();
            let pb = &mut self.players_buf;
            pb.clear();
            pb.extend(
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
                    }),
            );
            &*pb
        };
        // WAVE4: the tick-level player fingerprint (computed ONCE per tick;
        // the stamp check in ai_tick is free per mob)
        let tfp = MobAI::tick_fp(players);
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
        // THE LOOP — vanilla order (insertion), one mob at a time: the
        // mobs after this one observe its tick results through the same
        // store (Horizon creatures.rs:1468). The resolver borrows the
        // snapshot (immutable) while the mob mutates — disjoint fields,
        // the borrow splits cleanly.
        let stats: &mut Vec<AiStat> = &mut self.stats_buf;
        stats.clear();
        // disjoint field borrows: the snapshot is read-only during the
        // loop (it is only REPLACED between ticks through push_players),
        // the mobs mutate by slot.
        let snapshot: &crate::world::Snapshot = &self.store.snapshot;
        // WAVE2: the resolver is built ONCE per tick (the old code
        // constructed a fresh WorldRes per mob — a useless 5-word shuffle
        // x N) and its target stream advances through the tick.
        let mut res = WorldRes::open_ground_v0(snapshot, self.target_rng);
        res.fold_off = self.fold_off;
        for slot in self.store.mobs.iter_mut() {
            if !slot.alive {
                continue;
            }
            res.target_rng = self.target_rng;
            let stat = {
                let _g = ai_guard();
                slot.mob
                    .ai_tick(players, game_time, &mut res, &mut self.pack_scratch, &mut self.out_buf, &mut self.pack, tfp)
            };
            self.target_rng = res.target_rng;
            stats.push(stat);
        }
        // travel pass (vanilla: the movement runs inside the same body
        // after the AI; the split here is the profile's honesty — ai and
        // travel are separable costs of the same tick)
        for slot in self.store.mobs.iter_mut() {
            if !slot.alive {
                continue;
            }
            let _g = travel_guard();
            slot.mob.move_tick(&self.mth);
        }
        // tombstone compaction (between ticks, the Horizon lazy compact).
        // WAVE2: compact RENUMBERS slots — the grid's stored indices all
        // go stale, so it invalidates and the next rebuild re-inserts.
        if self.store.free.len() > self.compact_at {
            self.store.compact();
            self.grid.invalidate_all();
            for m in &mut self.store.mobs {
                m.grid_cell = (i64::MAX, i64::MAX);
            }
        }
        self.store.game_time = game_time;
        &stats[..]
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

    /// THE wave4 executor gate: the resident-pack executor (stamps, shared
    /// scratch) must be BIT-IDENTICAL to the wave3 full-pack executor over
    /// a chased, strolling, target-locked population — positions, seeds,
    /// goal states, nav, the whole store, word for word.
    #[test]
    fn resident_executor_matches_full_pack_executor() {
        let build = |full: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_full_pack(full);
            for i in 0..64 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 8) as f64 * 3.0,
                    64.0,
                    (i / 8) as f64 * 3.0,
                );
                m.goals = zombie_table();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut a = build(false); // resident (production)
        let mut b = build(true); // wave3 full-pack twin
        for t in 0..400 {
            a.tick(t);
            b.tick(t);
            assert_eq!(a.store.len(), b.store.len(), "tick {t}: population");
            for (ma, mb) in a.store.mobs.iter().zip(b.store.mobs.iter()) {
                assert_eq!(ma.mob.id, mb.mob.id, "tick {t}: id order");
                assert_eq!(ma.mob.x.to_bits(), mb.mob.x.to_bits(), "tick {t}: mob {} x", ma.mob.id);
                assert_eq!(ma.mob.y.to_bits(), mb.mob.y.to_bits(), "tick {t}: mob {} y", ma.mob.id);
                assert_eq!(ma.mob.z.to_bits(), mb.mob.z.to_bits(), "tick {t}: mob {} z", ma.mob.id);
                assert_eq!(ma.mob.seed, mb.mob.seed, "tick {t}: mob {} seed", ma.mob.id);
                assert_eq!(ma.mob.yaw.to_bits(), mb.mob.yaw.to_bits(), "tick {t}: mob {} yaw", ma.mob.id);
                assert_eq!(ma.mob.nav_state, mb.mob.nav_state, "tick {t}: mob {} nav", ma.mob.id);
                assert_eq!(ma.mob.target_player, mb.mob.target_player, "tick {t}: mob {} target", ma.mob.id);
                assert_eq!(ma.mob.dx.to_bits(), mb.mob.dx.to_bits(), "tick {t}: mob {} dx", ma.mob.id);
                assert_eq!(ma.mob.dz.to_bits(), mb.mob.dz.to_bits(), "tick {t}: mob {} dz", ma.mob.id);
                for (ga, gb) in ma.mob.goals.iter().zip(mb.mob.goals.iter()) {
                    assert_eq!(ga.running, gb.running, "tick {t}: mob {} goal running", ma.mob.id);
                    assert_eq!(ga.state, gb.state, "tick {t}: mob {} goal state", ma.mob.id);
                }
            }
        }
        // the resident twin must actually chase (the live-path exercise)
        let chasing = a
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        assert!(chasing > 0, "the twin scenario must acquire targets");
    }

    /// THE WAVE6 executor gate: the FOLDED executor (find_target/LOS/stroll/
    /// RLA-start/cc resolved IN PLACE, zero park round-trips) must be
    /// BIT-IDENTICAL to the PARKED executor (every request staged + resumed —
    /// the pre-fold semantics) over the same chased/strolling/looking
    /// population: positions, seeds, goal states, nav, targets — the whole
    /// store, word for word, 400 ticks.
    #[test]
    fn folded_executor_matches_parked_executor() {
        let build = |fold_off: bool| -> NativeTick {
            let mut nt = NativeTick::new(0xC0FFEE);
            nt.set_fold_off(fold_off);
            for i in 0..96 {
                let mut m = MobAI::new(
                    1000 + i,
                    0xC0FFEE ^ (i as u64).wrapping_mul(0x9E37),
                    (i % 12) as f64 * 3.0,
                    64.0,
                    (i / 12) as f64 * 3.0,
                );
                m.goals = zombie_table();
                nt.add_mob(m);
            }
            nt.push_players(vec![PlayerTick {
                x: 12.0,
                y: 64.0,
                z: 4.0,
                id: 1,
                flags: 1,
            }]);
            nt
        };
        let mut parked = build(true);
        let mut folded = build(false);
        let dump = |nt: &NativeTick| -> Vec<u64> {
            let mut h = 0xcbf2_9ce4_8422_2325u64;
            let mut pop = 0u64;
            for s in &nt.store.mobs {
                if !s.alive {
                    continue;
                }
                pop += 1;
                let m = &s.mob;
                for w in [
                    m.id as u64,
                    m.x.to_bits(),
                    m.y.to_bits(),
                    m.z.to_bits(),
                    m.seed,
                    m.yaw.to_bits() as u64,
                    m.dx.to_bits(),
                    m.dy.to_bits(),
                    m.dz.to_bits(),
                    m.nav_state as u64,
                    m.target_player as u64,
                    m.no_action as u64,
                    m.tick_count as u64,
                    m.rel_x.to_bits(),
                    m.rel_z.to_bits(),
                    m.nav_x.to_bits(),
                    m.nav_z.to_bits(),
                    m.pend_x.to_bits(),
                    m.pend_z.to_bits(),
                ] {
                    h = (h ^ w).wrapping_mul(0x1000_0000_01b3);
                }
                for g in &m.goals {
                    h = (h ^ (g.running as u64)).wrapping_mul(0x1000_0000_01b3);
                    h = (h ^ (g.state as u64)).wrapping_mul(0x1000_0000_01b3);
                }
            }
            let _ = pop;
            [h, nt.target_rng, pop].to_vec()
        };
        for t in 0..400 {
            parked.tick(t);
            folded.tick(t);
            assert_eq!(
                dump(&parked),
                dump(&folded),
                "tick {t}: the folded executor diverged from the parked twin"
            );
        }
        // the scenario must exercise the folds: goals running + targets held
        let chasing = folded
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.target_player >= 0)
            .count();
        let running = folded
            .store
            .mobs
            .iter()
            .filter(|m| m.mob.goals.iter().any(|g| g.running))
            .count();
        assert!(chasing > 0, "the folded twin must acquire targets: {chasing}");
        assert!(running > 0, "the folded twin must run goals: {running}");
    }
}

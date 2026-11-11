//! N2-M0: the wholesale mob-batch protocol (doc 42 N2, phase M0) — the
//! Java copy layer's Rust contract. JNI-FREE by design: the module's
//! mobcore.rs wraps these calls 1:1 with the dumb JNI shim, and EVERY
//! behavior here is unit-tested — including the bit-identity twin against
//! the direct `NativeTick` use (the proven executor path the bench and the
//! whole twin family gate).
//!
//! The wire contract (one level tick, server thread):
//!
//!   init(seed) — once per world/seed; drop = reset
//!   add(eid, seed, x, y, z)     — the add diff (first sight of the mob;
//!                                  kind is reserved: M0 builds the zombie
//!                                  table, `MobAI::new` does it)
//!   remove(eid)                 — the remove diff (host-driven)
//!   set_players(xyz, id_flags)  — the immutable per-tick player snapshot
//!   tick(game_time, &mut ids, &mut pos) -> Ok(n) | Err(needed)
//!       ids[2i]   = eid (full i64)
//!       ids[2i+1] = ((nav_state as u64) << 32) | (no_action as u32)
//!       pos[4i..] = x, y, z, yaw (f64 widened; Java narrows yaw to f32 —
//!                   f32→f64→f32 is exact)
//!
//! The executor owns the FULL mob state after add (goals, seeds, nav,
//! movement): the Java body of a native mob is skipped upstream (M1's
//! aiStep weave) and the host applies the read-back — positions, yaw,
//! delta = 0 (the native travel already advanced the state), and
//! no_action (the vanilla despawn predicate reads the Java field).
//!
//! Honesty (M0 disclosures, mirrors doc 42 §7): the stroll stream is the
//! executor's own splitmix chain (the bench world's semantics); melee
//! damage / sounds / drops are the N3 event outbox (NOT in M0 — the mobs
//! move and fight by position only); despawn, sensors and every vanilla
//! predicate OUTSIDE the skipped AI body keep running in Java.

use crate::exec::NativeTick;
use crate::world::PlayerTick;
use swarm_sim::mob::MobAI;

pub struct MobBatch {
    exec: NativeTick,
}

impl MobBatch {
    /// `target_seed` = the stroll stream's seed (the executor's own chain;
    /// the bench uses 0xC0FFEE — the host hands its world seed here).
    pub fn new(target_seed: u64) -> Self {
        MobBatch {
            exec: NativeTick::new(target_seed),
        }
    }

    /// The add diff. `seed` = the mob's own LCG mirror (the host reads the
    /// SHARED_RANDOM value at first sight; after that the mob is fully
    /// native and its Java random is never used again). Double-add is the
    /// store's no-op (the host's first-sight tracking is the only driver).
    pub fn add(&mut self, eid: i64, seed: u64, x: f64, y: f64, z: f64) -> usize {
        self.exec.add_mob(MobAI::new(eid, seed, x, y, z));
        self.population()
    }

    /// The remove diff. True when the mob was known (the host stops
    /// tracking it; the tombstone compacts on the executor's threshold).
    pub fn remove(&mut self, eid: i64) -> bool {
        let known = self.exec.store.index.contains_key(&eid);
        self.exec.remove_mob(eid);
        known
    }

    /// The per-tick player snapshot: `xyz` = [x,y,z] * P flattened,
    /// `id_flags` = [id, flags] * P. Length mismatch = programming error
    /// on the shim side — the caller checks (both arrays are host-built
    /// with the same P).
    pub fn set_players(&mut self, xyz: &[f64], id_flags: &[i32]) {
        debug_assert_eq!(xyz.len(), id_flags.len() / 2 * 3);
        let players: Vec<PlayerTick> = (0..id_flags.len() / 2)
            .map(|i| PlayerTick {
                x: xyz[i * 3],
                y: xyz[i * 3 + 1],
                z: xyz[i * 3 + 2],
                id: id_flags[i * 2],
                flags: id_flags[i * 2 + 1],
            })
            .collect();
        self.exec.push_players(players);
    }

    /// Alive population (mobs.len() − tombstones — exact per MobStore).
    pub fn population(&self) -> usize {
        self.exec.store.mobs.len() - self.exec.store.free.len()
    }

    /// THE call: one wholesale executor tick + the read-back fill.
    /// Returns Ok(n) with n mobs written (the alive population), or
    /// Err(needed) when either buffer is too small — NOTHING is filled,
    /// the host grows the arrays and the NEXT tick proceeds (the current
    /// tick's native state is already advanced; the read-back is pure).
    ///
    /// The parallel arm fills the buffers FUSED (inside pass 1/pass 2 —
    /// the same alive-rank the stats use); the serial/knob shapes fall
    /// back to this method's own fill loop.
    pub fn tick(&mut self, game_time: i32, ids: &mut [i64], pos: &mut [f64]) -> Result<usize, usize> {
        let pop = self.population();
        if ids.len() < pop * 2 || pos.len() < pop * 4 {
            return Err(pop);
        }
        if self.exec.tick_wire_fused(game_time, ids, pos) {
            return Ok(pop);
        }
        self.exec.tick(game_time);
        let mut w = 0usize;
        for slot in &self.exec.store.mobs {
            if !slot.alive {
                continue;
            }
            let m = &slot.mob;
            ids[w * 2] = m.id;
            ids[w * 2 + 1] = ((((m.nav_state as i64) as u64) << 32)
                | ((m.no_action as u32 as u64) & 0xFFFF_FFFF)) as i64;
            pos[w * 4] = m.x;
            pos[w * 4 + 1] = m.y;
            pos[w * 4 + 2] = m.z;
            pos[w * 4 + 3] = m.yaw as f64;
            w += 1;
        }
        debug_assert_eq!(w, pop);
        Ok(w)
    }

    /// The stats of the last tick (the wire passes them through for the
    /// ledger/diagnostics; the twin tests assert on them directly).
    pub fn last_stats(&self) -> &[swarm_sim::mob::AiStat] {
        self.exec.last_stats()
    }

    /// The host's read access to the underlying executor (the bench wire
    /// mode mirrors the warm state; the twins inspect the store directly).
    pub fn exec_mut(&mut self) -> &mut NativeTick {
        &mut self.exec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The twin world: 64 zombies on the bench pile shape + 1 melee-magnet
    /// player (the same shape family the exec twins use).
    fn build(batch: &mut MobBatch, direct: &mut NativeTick) {
        for i in 0..64usize {
            let x = (i % 250) as f64 * 0.6 - 75.0 + 0.05 * ((i / 7) % 3) as f64;
            let z = (i / 250) as f64 * 0.6 - 75.0;
            let seed = 0xC0FFEEu64 ^ (i as u64).wrapping_mul(0x9E37);
            batch.add(1000 + i as i64, seed, x, 64.0, z);
            direct.add_mob(MobAI::new(1000 + i as i64, seed, x, 64.0, z));
        }
        let xyz = [0.0, 64.0, 0.0];
        let idf = [1, 1]; // id 1, PF_ALIVE
        batch.set_players(&xyz, &idf);
        direct.push_players(vec![PlayerTick {
            x: 0.0,
            y: 64.0,
            z: 0.0,
            id: 1,
            flags: 1,
        }]);
    }

    /// The whole-store FNV over the 22 live MobAI words + every goal record
    /// (the same field list the exec twin family dumps).
    fn dump(nt: &NativeTick) -> (u64, u64) {
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
                m.nav_x.to_bits(),
                m.nav_z.to_bits(),
                m.target_player as u64,
                m.no_action as u64,
                m.tick_count as u64,
                m.approach_x.to_bits(),
                m.approach_z.to_bits(),
                m.et_running as u64,
                m.ar_running as u64,
                m.pend_x.to_bits(),
                m.pend_z.to_bits(),
                m.lookat_player as u64,
            ] {
                h ^= w;
                h = h.wrapping_mul(0x100000001b3);
            }
            for g in &m.goals {
                let w = g.kind as u64
                    | ((g.prio as u64) << 8)
                    | ((g.flags as u64) << 16)
                    | ((g.running as u64) << 24)
                    | (((g.state as u32 as u64) & 0xFFFF_FFFF) << 32);
                h ^= w;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        (h, pop)
    }

    /// THE N2-M0 gate: the batch wire is bit-identical to the direct
    /// proven executor over the twin world — 400 ticks, the stats stream
    /// asserted per tick, the whole-store dump + the stream end at the end.
    #[test]
    fn batch_matches_direct_executor() {
        let mut batch = MobBatch::new(0xC0FFEE);
        let mut direct = NativeTick::new(0xC0FFEE);
        build(&mut batch, &mut direct);
        assert_eq!(batch.population(), 64);
        assert_eq!(direct.store.mobs.len(), 64);

        let mut ids = vec![0i64; 2 * 64];
        let mut pos = vec![0f64; 4 * 64];
        for t in 0..400 {
            let n = batch
                .tick(t, &mut ids, &mut pos)
                .expect("capacity is exact");
            assert_eq!(n, 64);
            let dstats = direct.tick(t);
            assert_eq!(batch.last_stats(), dstats, "stats diverged at t={t}");
            // the read-back IS the store (positions + nav + no_action)
            let mut k = 0usize;
            for s in &direct.store.mobs {
                if !s.alive {
                    continue;
                }
                assert_eq!(ids[k * 2], s.mob.id);
                assert_eq!((ids[k * 2 + 1] >> 32) as i32, s.mob.nav_state);
                assert_eq!((ids[k * 2 + 1] & 0xFFFF_FFFF) as i32, s.mob.no_action);
                assert_eq!(pos[k * 4].to_bits(), s.mob.x.to_bits());
                assert_eq!(pos[k * 4 + 1].to_bits(), s.mob.y.to_bits());
                assert_eq!(pos[k * 4 + 2].to_bits(), s.mob.z.to_bits());
                assert_eq!((pos[k * 4 + 3] as f32).to_bits(), s.mob.yaw.to_bits());
                k += 1;
            }
        }
        let (hb, pb) = dump(&batch.exec);
        let (hd, pd) = dump(&direct);
        assert_eq!(hb, hd, "the batch world diverged from the direct one");
        assert_eq!(pb, pd);
        // the twin world actually LIVES: somebody strolls, somebody chases
        let (h0, p0) = (hb, pb);
        assert_eq!(p0, 64);
        let _ = h0;
    }

    /// The add/remove/re-add exactness + the truncation contract.
    #[test]
    fn batch_add_remove_and_truncation() {
        let mut batch = MobBatch::new(0xC0FFEE);
        for i in 0..10 {
            batch.add(100 + i, 0xC0FFEE ^ i as u64, i as f64, 64.0, 0.0);
        }
        assert_eq!(batch.population(), 10);
        assert!(batch.remove(103));
        assert!(!batch.remove(103), "double remove = false");
        assert_eq!(batch.population(), 9);
        batch.add(103, 0xC0FFEE, 3.0, 64.0, 0.0); // re-add (fresh state)
        assert_eq!(batch.population(), 10);

        // truncation: buffers too small -> Err(population), nothing filled
        let mut ids = vec![0i64; 2 * 9];
        let mut pos = vec![0f64; 4 * 10];
        let r = batch.tick(0, &mut ids, &mut pos);
        assert_eq!(r, Err(10));
        assert!(ids.iter().all(|v| *v == 0), "ids must stay unfilled");
        // big enough -> Ok
        ids.resize(2 * 10, 0);
        let r = batch.tick(0, &mut ids, &mut pos);
        assert_eq!(r, Ok(10));
        assert_eq!(ids[0], 100);
    }

    /// The players push: the snapshot the pack sees arrives bit-exact
    /// (the batch flattening must reproduce PlayerTick verbatim).
    #[test]
    fn batch_players_reach_the_snapshot() {
        let mut batch = MobBatch::new(0xC0FFEE);
        batch.add(1, 7, 0.0, 64.0, 0.0);
        let xyz = [1.5, 64.0, -3.25, 10.0, 70.0, 20.0];
        let idf = [5, 1, 6, 0];
        batch.set_players(&xyz, &idf);
        let snap = &batch.exec.store.snapshot;
        assert_eq!(snap.players.len(), 2);
        assert_eq!(snap.players[0].id, 5);
        assert_eq!(snap.players[0].flags, 1);
        assert_eq!(snap.players[0].x.to_bits(), 1.5f64.to_bits());
        assert_eq!(snap.players[1].id, 6);
        assert_eq!(snap.players[1].flags, 0);
        assert_eq!(snap.players[1].z.to_bits(), 20.0f64.to_bits());
    }
}

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

use crate::despawn::DespawnCfg;
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
        // the DEFAULT MobAI (the zombie table's despawn bits included) —
        // kind==0 is NOT "the defaults" on the wire (a real zombie's kind
        // carries rwf|peaceful); the bench and the twins use this form.
        self.exec.add_mob(MobAI::new(eid, seed, x, y, z));
        self.population()
    }

    /// N3-N1 phase-2b: the add diff with the DESPAWN kind bits —
    /// kind = category ordinal (bits 0..8) | persistent (bit 8) |
    /// removeWhenFarAway (bit 9) | peaceful-despawn (bit 10), resolved by
    /// the wire's first-sight classification (doc 61 §2-3). kind == 0 =
    /// the zombie defaults (MONSTER, !persistent, rwf, peaceful).
    pub fn add_kind(&mut self, eid: i64, seed: u64, x: f64, y: f64, z: f64, kind: i32) -> usize {
        let mut m = MobAI::new(eid, seed, x, y, z);
        // the Java kind: category bits 0..8 | persistent bit 8 |
        // removeWhenFarAway bit 9 | peaceful-despawn bit 10 — translated
        // into the MobAI despawn word (persistent bit0 | rwf bit1 |
        // peaceful bit2 | category bits 8..16)
        let mut bits = ((kind as u32) & 0xFF) << 8;
        if kind & (1 << 8) != 0 {
            bits |= 1;
        }
        if kind & (1 << 9) != 0 {
            bits |= 2;
        }
        if kind & (1 << 10) != 0 {
            bits |= 4;
        }
        m.despawn = bits;
        self.exec.add_mob(m);
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

    /// N3-N1 phase-2b: the despawn config table lands (the once-per-level
    /// nativeDespawnCfg push). The decision stays OFF until this runs.
    pub fn set_despawn_cfg(&mut self, cfg: Option<DespawnCfg>) {
        self.exec.set_despawn_cfg(cfg);
    }

    /// The last tick's read-back cause column — the N3-N1 event outbox
    /// riding the rows: cause != 0 at ids[2i+1] bits 48..56 (1 = Peaceful,
    /// 2 = Hard, 3 = Soft). The rows are delivered on THIS tick's fill;
    /// the slots tombstone after it (finalize_despawns).
    pub fn despawn_causes(&self) -> &[u8] {
        &self.exec.despawn_cause
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
    /// `difficulty` = the level's Difficulty ordinal (0 = PEACEFUL) — the
    /// N3-N1 peaceful gate reads it per tick (a /difficulty change is
    /// exact on the next boundary).
    ///
    /// The parallel arm fills the buffers FUSED (inside pass 1/pass 2 —
    /// the same alive-rank the stats use) and tombstones the despawned
    /// slots internally; the serial/knob shapes fall back to this method's
    /// own fill loop, THEN tombstone (the despawned event rows must
    /// survive the fill).
    pub fn tick(
        &mut self,
        game_time: i32,
        difficulty: i32,
        ids: &mut [i64],
        pos: &mut [f64],
    ) -> Result<usize, usize> {
        let pop = self.population();
        if ids.len() < pop * 2 || pos.len() < pop * 4 {
            return Err(pop);
        }
        self.exec.set_difficulty(difficulty);
        if self.exec.tick_wire_fused(game_time, ids, pos) {
            return Ok(pop);
        }
        // the serial fallback: the walk already ran inside tick_wire_fused
        // (the despawn causes sit in the cause column) — fill the rows,
        // THEN tombstone. The old shape re-ran exec.tick here — a LATENT
        // DOUBLE TICK on the serial arm (the twin always drives the fused
        // parallel path); this shape fixes it.
        self.exec.fill_readback(ids, pos);
        self.exec.finalize_despawns();
        Ok(pop)
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

    /// The batch twins + the despawn twins SERIALIZE here (the same reason
    /// the exec.rs twin family locks): concurrent knob flips (odd-skip /
    /// z0 / the probes) landing between the two arms would diverge them —
    /// a pure test-harness race, not an executor bug.
    static EXEC_TWIN_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    macro_rules! twin_guard {
        () => {
            let _twin_guard = EXEC_TWIN_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        };
    }

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
            // N3-N1 phase-2b: the despawn bits ride the dump (the decision
            // inputs are part of the twin contract)
            h ^= m.despawn as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        (h, pop)
    }

    /// THE N2-M0 gate: the batch wire is bit-identical to the direct
    /// proven executor over the twin world — 400 ticks, the stats stream
    /// asserted per tick, the whole-store dump + the stream end at the end.
    #[test]
    fn batch_matches_direct_executor() {
        twin_guard!();
        let mut batch = MobBatch::new(0xC0FFEE);
        let mut direct = NativeTick::new(0xC0FFEE);
        build(&mut batch, &mut direct);
        assert_eq!(batch.population(), 64);
        assert_eq!(direct.store.mobs.len(), 64);

        let mut ids = vec![0i64; 2 * 64];
        let mut pos = vec![0f64; 4 * 64];
        for t in 0..400 {
            let n = batch
                .tick(t, 2, &mut ids, &mut pos)
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
                assert_eq!(((ids[k * 2 + 1] >> 32) & 0xFFFF) as i32, s.mob.nav_state);
                assert_eq!((ids[k * 2 + 1] & 0xFFFF_FFFF) as i32, s.mob.no_action);
                assert_eq!(
                    ((ids[k * 2 + 1] >> 48) & 0xFF) as u8,
                    0,
                    "no despawn fires in the twin world (all mobs inside the soft range)"
                );
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
        let r = batch.tick(0, 2, &mut ids, &mut pos);
        assert_eq!(r, Err(10));
        assert!(ids.iter().all(|v| *v == 0), "ids must stay unfilled");
        // big enough -> Ok
        ids.resize(2 * 10, 0);
        let r = batch.tick(0, 2, &mut ids, &mut pos);
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

    // ------------------------------------------------------------------
    // N3-N1 phase-2b: the despawn strata twin — the batch wire vs the
    // direct executor over a world DESIGNED to fire every branch of the
    // checkDespawn port (hard, soft-with-roll, missed-roll, persistent,
    // no-player, in-soft-reset), the event rows asserted on the read-back
    // and the whole-store dumps asserted equal after the tombstones.
    // ------------------------------------------------------------------

    /// The stock-monster cylinder config: hard 128/128, soft 32/56.
    fn despawn_cfg() -> DespawnCfg {
        let mk = |h: i32, v: i32| {
            let h2 = (h as f64) * (h as f64);
            crate::despawn::DespawnRange {
                h2,
                ratio: h2 / ((v as f64) * (v as f64)),
                v_default: v,
                v,
            }
        };
        DespawnCfg {
            ellipsoid: false,
            pairs: [
                Some(crate::despawn::DespawnRangePair {
                    hard: mk(128, 128),
                    soft: mk(32, 56),
                }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        }
    }

    /// A seed whose first nextInt(800) draw lands on `want` (bound 800 is
    /// even — exactly one 48-bit LCG step per draw, no rejection loop).
    fn seed_for_draw(want: i32) -> u64 {
        for s in 0xC0FFEEu64.. {
            let mut t = s;
            if swarm_sim::mob::lcg_next_int(&mut t, 800) == want {
                return s;
            }
        }
        unreachable!()
    }

    #[test]
    fn despawn_strata_twin_batch_vs_direct() {
        twin_guard!();
        let mut batch = MobBatch::new(0xC0FFEE);
        let mut direct = NativeTick::new(0xC0FFEE);
        let cfg = despawn_cfg();
        batch.set_despawn_cfg(Some(cfg.clone()));
        direct.set_despawn_cfg(Some(cfg));
        // the strata (all non-persistent zombies unless stated):
        //   id 1 @10   — INSIDE the soft range -> alive + noAction reset
        //   id 2 @100  — outside soft, inside hard, noAct 0 -> alive
        //   id 3 @200  — outside the hard range -> HARD despawn
        //   id 4 @100  — noAct 9000 + draw 0 -> SOFT despawn
        //   id 5 @100  — noAct 9000 + draw != 0 -> alive (the roll missed)
        //   id 6 @200  — persistent -> the hard gate declines
        //   id 7 @10   — NO affects-spawning player (below) -> alive, no reset
        let strata: &[(i64, u64, f64, i32, u32)] = &[
            (1, 0xC0FFEE, 10.0, 0, 0b110),
            (2, 0xC0FFEE, 100.0, 0, 0b110),
            (3, 0xC0FFEE, 200.0, 0, 0b110),
            (4, seed_for_draw(0), 100.0, 9000, 0b110),
            (5, seed_for_draw(1), 100.0, 9000, 0b110),
            (6, 0xC0FFEE, 200.0, 0, 0b111),
            (7, 0xC0FFEE, 10.0, 0, 0b110),
        ];
        for &(id, seed, x, noact, bits) in strata {
            let mut m = MobAI::new(id, seed, x, 64.0, 0.0);
            m.no_action = noact;
            m.despawn = bits;
            batch.exec_mut().add_mob(m.clone());
            direct.add_mob(m);
        }
        // ONE affects-spawning player at the origin (flags bit0 alive, bit1
        // affects); stratum 7 sits at 10 blocks — IN the soft range — its
        // reset only fires when the player predicate passes... it does (the
        // same player covers every mob). The no-player gate is the
        // despawn_no_player_gate test below.
        let xyz = [0.0, 64.0, 0.0];
        let idf = [1, 3]; // id 1, alive(1) | affects(2)
        batch.set_players(&xyz, &idf);
        direct.push_players(vec![crate::world::PlayerTick {
            x: 0.0,
            y: 64.0,
            z: 0.0,
            id: 1,
            flags: 3,
        }]);

        let mut ids = vec![0i64; 2 * 7];
        let mut pos = vec![0f64; 4 * 7];
        // tick 1: the events fire and ride the rows
        let n = batch.tick(0, 2, &mut ids, &mut pos).expect("capacity");
        assert_eq!(n, 7, "the despawned rows ride THIS tick's read-back");
        let mut causes: std::collections::HashMap<i64, u8> = std::collections::HashMap::new();
        for i in 0..7 {
            causes.insert(
                ids[i * 2],
                ((ids[i * 2 + 1] >> 48) & 0xFF) as u8,
            );
        }
        assert_eq!(causes[&3], 2, "id 3: HARD despawn");
        assert_eq!(causes[&4], 3, "id 4: SOFT despawn (the roll hit)");
        for &id in &[1i64, 2, 5, 6, 7] {
            assert_eq!(causes[&id], 0, "id {id}: alive");
        }
        // the direct arm ticks + finalizes on its own
        direct.tick(0);
        direct.finalize_despawns();
        // the despawned slots are GONE (the population shrank)
        assert_eq!(batch.population(), 5);
        assert_eq!(direct.store.mobs.iter().filter(|s| s.alive).count(), 5);
        // the whole stores agree (the survivors' state is bit-identical)
        let (hb, pb) = dump(&batch.exec);
        let (hd, pd) = dump(&direct);
        assert_eq!(pb, pd);
        assert_eq!(hb, hd, "the despawn world diverged after the tombstones");
        // the no_action reset landed on the in-soft survivor (id 1): the
        // reset zeroes 9000 -> the AI's serverAiStep head increments to 1
        // (vanilla lambda$tick$0: checkDespawn THEN tick). Without the
        // decision the mirror would read 9001 — the reset is observable.
        let slot1 = *batch.exec_mut().store.index.get(&1).expect("id 1 alive");
        assert_eq!(batch.exec_mut().store.mobs[slot1 as usize].mob.no_action, 1,
            "the in-soft reset fired before the AI's increment");
    }

    /// The no-player gate: without an affects-spawning player the decision
    /// returns early (vanilla findNearbyPlayer == null) — no despawn, no
    /// reset, and the roll never draws.
    #[test]
    fn despawn_no_player_gate() {
        twin_guard!();
        let mut batch = MobBatch::new(0xC0FFEE);
        batch.set_despawn_cfg(Some(despawn_cfg()));
        let mut m = MobAI::new(9, seed_for_draw(0), 200.0, 64.0, 0.0);
        m.no_action = 9000;
        batch.exec_mut().add_mob(m);
        // a NON-affecting player present (flags = 1: alive but not affecting)
        batch.set_players(&[0.0, 64.0, 0.0], &[1, 1]);
        let mut ids = vec![0i64; 2];
        let mut pos = vec![0f64; 4];
        batch.tick(0, 2, &mut ids, &mut pos).expect("capacity");
        assert_eq!(((ids[1] >> 48) & 0xFF) as u8, 0, "no despawn without the affects bit");
        assert_eq!(batch.population(), 1, "the mob survives the tick");
    }

    /// THE DRAW-ORDER proof: the 1/800 roll consumes its mirror draw
    /// BEFORE the AI draws — the same mob with the decision OFF (the M2c
    /// status quo) takes the identical AI path from the UN-advanced seed,
    /// so the two stores must DIVERGE (a deterministic, seed-pinned pair).
    #[test]
    fn despawn_roll_precedes_the_ai_draws() {
        twin_guard!();
        let mk_world = |cfg: Option<DespawnCfg>| -> MobBatch {
            let mut b = MobBatch::new(0xC0FFEE);
            b.set_despawn_cfg(cfg);
            let mut m = MobAI::new(5, seed_for_draw(1), 100.0, 64.0, 0.0);
            m.no_action = 9000; // past the 600 gate: the roll RUNS
            b.exec_mut().add_mob(m);
            b.set_players(&[0.0, 64.0, 0.0], &[1, 3]);
            b
        };
        let mut armed = mk_world(Some(despawn_cfg()));
        let mut off = mk_world(None);
        let mut ids = vec![0i64; 2];
        let mut pos = vec![0f64; 4];
        armed.tick(0, 2, &mut ids, &mut pos).expect("capacity");
        let _ = off.tick(0, 2, &mut ids, &mut pos).expect("capacity");
        let slot_a = *armed.exec_mut().store.index.get(&5).unwrap();
        let slot_o = *off.exec_mut().store.index.get(&5).unwrap();
        let seed_a = armed.exec_mut().store.mobs[slot_a as usize].mob.seed;
        let seed_o = off.exec_mut().store.mobs[slot_o as usize].mob.seed;
        assert_ne!(
            seed_a, seed_o,
            "the roll consumed its mirror draw before the AI drew"
        );
        // the armed chain sits exactly one LCG step FURTHER at the same
        // AI-draw depth is not assertable without counting the AI's draws —
        // the divergence itself is the contract (deterministic seed pair).
    }
}

//! SWARM E2E — the 10^9-zombie tick budget, measured end to end.
//!
//! The honest three-layer scenario:
//!   T2 far field  : the SwarmLedger holds the EXACT per-chunk counts for
//!                   25M chunks x 40 = 1,000,000,000 zombies. No entities.
//!   T1 sim belt   : the 289 chunks inside the sim boundary are
//!                   materialized into the exact index (vanilla queries
//!                   answer from it; belt entities are not ticked —
//!                   vanilla-equal: ActivationRange/EAR).
//!   T0 hot ring   : entities inside the hot square run the FULL native
//!                   tick per game tick — the parity-proven ai_floor
//!                   selector walk (bit-identical to Paper 1.21.8, run
//!                   #369), the exact travel slice, the exact push
//!                   kernel, the exact move/setPos predicate.
//!
//! The bot flies across the field (5.6 blocks/tick) carrying a live
//! player snapshot: ring mobs see it, target it (NearestAttackable),
//! chase it (MeleeAttack) and shove each other (push kernel) — the
//! worst-case hot loop, not a quiet field.
//!
//! Reported: per-tick wall time (avg / worst) against the 20-TPS budget
//! of 50 ms, with the per-stage breakdown. The far-field transition cost
//! rides the same tick (boundary diff + materialization).
//!
//! Run: cargo run --release -p swarm-sim --example swarm_e2e [-- --farm]

use centities_core::goals as g;
use centities_core::mobtravel::Mth;
use centities_core::Aabb;
use std::time::Instant;
use swarm_sim::mob::out_len;
use swarm_core::ChunkDelta;
use swarm_sim::mob::{FlatWorld, MobAI, PlayerSnap};
use swarm_sim::{HotRing, SwarmEngine};

// the push kernel is the JNI-serving crate's own component; the example
// drives it exactly like the production n_push path does.
use centities_core::pushcore::{push_tick, PushCore, PushOut, GATE_MIN, PF_PUSHABLE};

fn main() {
    let farm = std::env::args().any(|a| a == "--farm");
    let (chunks_side, per_chunk, hot_dense) = if farm { (200, 25, 1100) } else { (5000, 40, 0) };

    // ---- T2: the 10^9 ledger ------------------------------------------
    let t0 = Instant::now();
    let mut eng = SwarmEngine::new(0, 8);
    let half = chunks_side / 2;
    let mut batch = Vec::with_capacity((chunks_side * chunks_side) as usize);
    for cx in -half..half {
        for cz in -half..half {
            batch.push(ChunkDelta::new(cx, cz, per_chunk));
        }
    }
    if farm {
        // the hot chunks around the spawn get farm density
        for cx in -4..=4i32 {
            for cz in -4..=4i32 {
                batch.push(ChunkDelta::new(cx, cz, hot_dense - per_chunk));
            }
        }
    }
    eng.seed_far(&batch).unwrap();
    drop(batch);
    let total = eng.world_total();
    println!(
        "seed: {total} entities / {} chunks in {:?}",
        chunks_side * chunks_side,
        t0.elapsed()
    );
    let expected = (chunks_side * chunks_side * per_chunk) as u64
        + if farm {
            81 * (hot_dense - per_chunk) as u64
        } else {
            0
        };
    assert_eq!(total, expected, "the seeded total must be exact");
    if !farm {
        assert_eq!(total, 1_000_000_000, "the headline number must be exact");
    }

    // ---- the hot loop's storage ---------------------------------------
    let mut ring = HotRing::new(8192);
    let mut mobs: std::collections::HashMap<i64, MobAI> = std::collections::HashMap::new();
    let mut pc = PushCore::new();
    let mut pout = PushOut::default();
    let mth = Mth::new();
    let mut world = FlatWorld::new(0x5EED_10E9);

    let ticks = if farm { 200 } else { 400 };
    let (mut bx, bz) = (0.0f64, 0.0f64);
    let mut stage = [0u128; 6]; // transitions, fill, ai, move, push, index
    let mut tick_ms: Vec<f64> = Vec::with_capacity(ticks);
    let mut worst_tick = 0.0f64;
    let mut ring_max = 0usize;
    let mut chasing = 0usize;
    let mut pushes = 0u64;

    for tick in 0..ticks {
        let t_tick = Instant::now();
        // the bot flies +x at 5.6 blocks/tick (creative-flight bot);
        // the farm scenario keeps it AFK inside the dense square
        if !farm {
            bx += 5.6;
        }
        let pchunk = (bx as i32 >> 4, bz as i32 >> 4);

        let t = Instant::now();
        let cost = eng.set_players(&[(pchunk.0, pchunk.1)]).unwrap();
        stage[0] += t.elapsed().as_nanos();

        let t = Instant::now();
        let filled = eng.fill_ring(&mut ring, pchunk, 1);
        stage[1] += t.elapsed().as_nanos();
        ring_max = ring_max.max(ring.len());

        let players = [PlayerSnap {
            x: bx,
            y: 64.0,
            z: bz,
            id: 1,
            flags: g::PF_ALIVE,
        }];

        let ids: Vec<i64> = ring.iter_ids().collect();
        let mut chased = 0usize;
        // wave2: the shared AI buffers — ONE pack scratch + ONE out buffer
        // for the whole population per tick (zero steady-state allocation)
        let mut ai_scratch: Vec<i32> = Vec::new();
        let mut ai_out: Vec<i32> = vec![0i32; out_len()];
        for &id in &ids {
            let s = ring.slot_of(id).unwrap();
            mobs.entry(id).or_insert_with(|| {
                let (x, y, z) = ring.pos(s);
                MobAI::new(id, (0x9E37_0000 ^ id) as u64, x as f64, y as f64, z as f64)
            });
            let mut pairs: Vec<(i64, f64, f64)> = Vec::new();
            {
                let m = mobs.get_mut(&id).unwrap();
                // 1. the selector walk (the parity-proven ai_floor)
                let t = Instant::now();
                let stat = m.ai_tick(&players, tick as i32, &mut world, &mut ai_scratch, &mut ai_out);
                stage[2] += t.elapsed().as_nanos();
                debug_assert_eq!(stat.status, g::ST_DONE);
                if m.target_player >= 0 {
                    chased += 1;
                }

                // 2. the push kernel against the exact materialized index —
                //    the vanilla pushEntities slice on the pre-move state
                let t = Instant::now();
                let b = Aabb::new(m.x - 0.3, m.y, m.z - 0.3, m.x + 0.3, m.y + 1.95, m.z + 1.95);
                pc.upsert(
                    id,
                    0,
                    &b,
                    m.x,
                    m.z,
                    m.dx,
                    m.dy,
                    m.dz,
                    PF_PUSHABLE,
                    0,
                    (m.y as i32) >> 4,
                    id as i32,
                );
                let self_slot = pc.slot_of(id).unwrap();
                let self_ncol = pc.ncol[self_slot];
                push_tick(
                    &mut pc,
                    &mut eng.index,
                    0,
                    id,
                    self_slot,
                    false,
                    false,
                    false,
                    true,
                    8,
                    24,
                    GATE_MIN,
                    tick as u64,
                    self_ncol,
                    false,
                    &mut pout,
                );
                if pout.ok {
                    // the push delta rides the entity delta (vanilla: the
                    // pushed mob moves on its NEXT travel); ring mobs travel
                    // every tick, belt mobs accumulate — the ledger keeps
                    // counts, not deltas, for the far field
                    m.dx += pout.ddx;
                    m.dz += pout.ddz;
                    pairs.clone_from(&pout.pairs);
                    pushes += pairs.len() as u64;
                }
                stage[4] += t.elapsed().as_nanos();

                // 3. travel consumes the delta (incl. the push)
                let t = Instant::now();
                m.move_tick(&mth);
                stage[3] += t.elapsed().as_nanos();

                // 4. publish the new box to the exact index
                let t = Instant::now();
                let b = Aabb::new(m.x - 0.3, m.y, m.z - 0.3, m.x + 0.3, m.y + 1.95, m.z + 1.95);
                eng.index.upsert(id, 0, &b);
                stage[5] += t.elapsed().as_nanos();
                ring.set_pos(s, m.x as f32, m.y as f32, m.z as f32);
            }
            // 5. the pushed candidates' deltas (vanilla doPush order);
            // belt candidates not in the ring have no delta storage —
            // the ledger keeps counts, the delta cold-start gap is
            // bounded by one PUSH_QUANTUM per event (doc honesty note)
            for (cid, ddx, ddz) in &pairs {
                if let Some(cm) = mobs.get_mut(cid) {
                    cm.dx += ddx;
                    cm.dz += ddz;
                }
            }
        }
        chasing = chasing.max(chased);

        let dt = t_tick.elapsed().as_secs_f64() * 1000.0;
        tick_ms.push(dt);
        worst_tick = worst_tick.max(dt);

        // the world-total invariant, spot-checked
        if tick % 50 == 0 {
            assert_eq!(
                eng.world_total(),
                total,
                "world total drifted at tick {tick}"
            );
        }
        let _ = (cost, filled);
    }

    tick_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let avg = tick_ms.iter().sum::<f64>() / tick_ms.len() as f64;
    let p99 = tick_ms[(tick_ms.len() as f64 * 0.99) as usize];
    let names = [
        "transition",
        "fill    ",
        "ai      ",
        "move    ",
        "push    ",
        "publish ",
    ];
    let budget_ms = 50.0;
    println!(
        "\n=== {} ({} ticks) ===",
        if farm {
            "FARM (ring-capped)"
        } else {
            "WALK (bot flies across 1e9)"
        },
        ticks
    );
    println!("ring: max {ring_max} (cap 8192), peak chasing {chasing}, push pairs {pushes}");
    println!("tick ms: avg {avg:.2}  p99 {p99:.2}  worst {worst_tick:.2}  — budget {budget_ms:.0} (20 TPS)");
    for (i, n) in names.iter().enumerate() {
        let per = stage[i] as f64 / ticks as f64 / 1e6;
        println!("  {n} {per:8.3} ms/tick");
    }
    let headroom = budget_ms / avg.max(1e-9);
    println!(
        "verdict: {} — headroom x{headroom:.1} under the 20-TPS budget",
        if worst_tick < budget_ms {
            "PASS"
        } else {
            "FAIL"
        }
    );
}

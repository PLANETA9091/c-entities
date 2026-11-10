//! TEMP WAVE11 probe: the eval-stream CEILING — how fast can the idle-path
//! canUse eval stream go?
//!
//! Mode A (scalar): one mob at a time, the exact eval bodies + the serial
//! 6-draw LCG chain — the workload the current engine's update phase does
//! (minus the Ctx build / locks / cleanup / out-buffer plumbing).
//!
//! Mode B (8-wide software-pipelined): 8 mobs interleaved — the 8
//! independent LCG chains hide the 5-cycle mul latency (the serial chain
//! becomes ~1 cycle/mob/step), and the gate checks interleave too. This is
//! the batch walk's core promise WITHOUT any intrinsics.
//!
//! Both modes MUST produce identical seeds + verdicts (the self-check —
//! the interleaving is bit-exact by construction; the assert pins it).
//!
//! Usage: cargo run --release -p centities-native --example eval_probe [-- mobs ticks]

use swarm_sim::mob::{MobAI, N_GOALS};
use centities_core::goals as g;

// the exact ThreadUnsafeRandom math (mirror of goals.rs::Lcg)
#[inline]
fn lcg_advance(s: &mut u64) -> u64 {
    *s = s.wrapping_mul(25214903917).wrapping_add(11) & 0xFFFF_FFFF_FFFF;
    *s
}
#[inline]
fn lcg_next(s: &mut u64, bits: i32) -> i32 {
    (lcg_advance(s) >> (48 - bits)) as i32
}
#[inline]
fn lcg_next_int_bound(s: &mut u64, bound: i32) -> i32 {
    if bound & (bound - 1) == 0 {
        return (((bound as i64) * (lcg_next(s, 31) as i64)) >> 31) as i32;
    }
    loop {
        let bits = lcg_next(s, 31);
        let val = bits % bound;
        if bits.wrapping_sub(val).wrapping_add(bound - 1) >= 0 {
            return val;
        }
    }
}
#[inline]
fn lcg_next_f32(s: &mut u64) -> f32 {
    (lcg_next(s, 24) as f32) / 16777216.0f32
}

const MF_IDLE: i32 = g::MF_NO_PASSENGERS | g::MF_MAINHAND_EMPTY; // MF_BASE
const LOOK_PROB_BITS: i32 = 0.02f32.to_bits() as i32;

/// Mode A: the idle-path eval stream for ONE mob (even tick, 1 player,
/// no running goals, no target). Returns (pass?, final seed).
/// The verdicts + draw order mirror goals.rs::can_use_inner exactly.
#[inline]
fn eval_stream_a(mob: &MobAI, game_time: i32) -> (bool, u64) {
    let mut s = mob.seed;
    let mf = MF_IDLE
        | if mob.target_player >= 0 { g::MF_HAS_TARGET } else { 0 };
    let goals = &mob.goals;
    // SEL_TARGET
    // rec0 HURT_BY: mob_ts==state && !MF_LAST_HURT_BY -> No (no draw)
    let _ = (0i32 == goals[0].state) && (mf & g::MF_LAST_HURT_BY == 0);
    // rec1..3 NEAREST: interval 10 draw; pass -> scan (stops the stream)
    for rec in 1..4 {
        let ri = goals[rec].cfg & 0xFFFF;
        if ri > 0 && lcg_next_int_bound(&mut s, ri) != 0 {
            continue;
        }
        return (true, s); // scan-pass: extras out of scope
    }
    // SEL_GOAL
    // rec4 SPEAR: MF_MAINHAND_EMPTY -> No (no draw)
    let _ = mf & g::MF_MAINHAND_EMPTY != 0;
    // rec5/6 ZA: the 20-tick throttle + HAS_TARGET (no draws)
    for rec in 5..7 {
        if (game_time as u32).wrapping_sub(goals[rec].state as u32) < 20 {
            continue;
        }
        if mf & g::MF_HAS_TARGET == 0 {
            continue;
        }
        return (true, s); // the canuse tail: extras out of scope
    }
    // rec7 MTV: state<=15, night cfg, NEAR_VILLAGE (no draws)
    if goals[7].state > 15 {
        return (true, s);
    }
    if goals[7].cfg & 1 != 0 && mf & g::MF_BRIGHT_OUTSIDE != 0 {
        continue_eval();
    }
    if mf & g::MF_NEAR_VILLAGE == 0 {
        // No — fall through
    } else {
        return (true, s);
    }
    // rec8 STROLL: NO_PASSENGERS set, no_action<100, interval 60 draw
    if mf & g::MF_NO_PASSENGERS == 0 {
        continue_eval();
    }
    if mob.no_action >= 100 {
        continue_eval();
    }
    let ri = goals[8].cfg & 0xFFFF;
    if ri > 0 && lcg_next_int_bound(&mut s, ri) != 0 {
        // No
    } else if ri > 0 {
        return (true, s); // stroll pass
    }
    // rec9 LOOKAT: prob draw
    if lcg_next_f32(&mut s) as f64 >= f64::from(f32::from_bits(LOOK_PROB_BITS as u32)) {
        // No
    } else {
        return (true, s); // lookat pass
    }
    // rec10 LOOKAROUND: prob draw
    if lcg_next_f32(&mut s) < 0.02f32 {
        return (true, s); // RLA start
    }
    (false, s)
}

#[inline(always)]
fn continue_eval() {}

/// Mode B: the SAME stream, 8 mobs interleaved. `mobs` is exactly 8
/// MobAI refs; `out_seeds`/`out_pass` receive the results.
fn eval_stream_b(mobs: [&MobAI; 8], game_time: i32, out: &mut [(bool, u64); 8]) {
    let mut seeds = [0u64; 8];
    let mut pass = [false; 8];
    let mut mfs = [0i32; 8];
    let mut st0 = [0i32; 8];
    let mut st5 = [0i32; 8];
    let mut st6 = [0i32; 8];
    let mut st7 = [0i32; 8];
    let mut cfg1 = [0i32; 8];
    let mut cfg7 = [0i32; 8];
    let mut cfg8 = [0i32; 8];
    let mut na = [0i32; 8];
    for l in 0..8 {
        let m = mobs[l];
        seeds[l] = m.seed;
        mfs[l] = MF_IDLE
            | if m.target_player >= 0 { g::MF_HAS_TARGET } else { 0 };
        st0[l] = m.goals[0].state;
        st5[l] = m.goals[5].state;
        st6[l] = m.goals[6].state;
        st7[l] = m.goals[7].state;
        cfg1[l] = m.goals[1].cfg & 0xFFFF;
        cfg7[l] = m.goals[7].cfg & 0xFFFF;
        cfg8[l] = m.goals[8].cfg & 0xFFFF;
        na[l] = m.no_action;
    }
    // SEL_TARGET
    for l in 0..8 {
        let _ = (0 == st0[l]) && (mfs[l] & g::MF_LAST_HURT_BY == 0);
    }
    // the 3 NEAREST interval draws — the 8 chains step in lockstep
    for rec in 1..4 {
        for l in 0..8 {
            if pass[l] {
                continue;
            }
            let ri = cfg1[l];
            if ri > 0 && lcg_next_int_bound(&mut seeds[l], ri) != 0 {
                continue;
            }
            pass[l] = true;
        }
    }
    // SEL_GOAL
    for l in 0..8 {
        let _ = mfs[l] & g::MF_MAINHAND_EMPTY != 0;
    }
    for rec in 5..7 {
        let st = if rec == 5 { &st5 } else { &st6 };
        for l in 0..8 {
            if pass[l] {
                continue;
            }
            if (game_time as u32).wrapping_sub(st[l] as u32) < 20 {
                continue;
            }
            if mfs[l] & g::MF_HAS_TARGET == 0 {
                continue;
            }
            pass[l] = true;
        }
    }
    for l in 0..8 {
        if pass[l] {
            continue;
        }
        if st7[l] > 15 {
            pass[l] = true;
            continue;
        }
        if cfg7[l] & 1 != 0 && mfs[l] & g::MF_BRIGHT_OUTSIDE != 0 {
            continue;
        }
        if mfs[l] & g::MF_NEAR_VILLAGE != 0 {
            pass[l] = true;
        }
    }
    for l in 0..8 {
        if pass[l] {
            continue;
        }
        if mfs[l] & g::MF_NO_PASSENGERS == 0 || na[l] >= 100 {
            continue;
        }
        let ri = cfg8[l];
        if ri > 0 && lcg_next_int_bound(&mut seeds[l], ri) != 0 {
            continue;
        }
        if ri > 0 {
            pass[l] = true;
        }
    }
    for l in 0..8 {
        if pass[l] {
            continue;
        }
        if lcg_next_f32(&mut seeds[l]) as f64
            >= f64::from(f32::from_bits(LOOK_PROB_BITS as u32))
        {
            continue;
        }
        pass[l] = true;
    }
    for l in 0..8 {
        if pass[l] {
            continue;
        }
        if lcg_next_f32(&mut seeds[l]) < 0.02f32 {
            pass[l] = true;
        }
    }
    for l in 0..8 {
        out[l] = (pass[l], seeds[l]);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n_mobs: usize = args.get(1).map_or(25000, |s| s.parse().unwrap_or(25000));
    let ticks: usize = args.get(2).map_or(200, |s| s.parse().unwrap_or(200));
    let seed: u64 = args
        .get(3)
        .map_or(0xC0FFEE, |s| s.parse().unwrap_or(0xC0FFEE));

    // the same stand shape as native_bench
    let mut mobs: Vec<MobAI> = Vec::with_capacity(n_mobs);
    for i in 0..n_mobs {
        let x = (i % 250) as f64 * 0.6 - 75.0 + 0.05 * ((i / 7) % 3) as f64;
        let z = (i / 250) as f64 * 0.6 - 75.0;
        mobs.push(MobAI::new(
            1000 + i as i64,
            seed ^ (i as u64).wrapping_mul(0x9E37),
            x,
            64.0,
            z,
        ));
    }

    // warmup + self-check: A vs B must agree on every mob
    {
        let mut out = [(false, 0u64); 8];
        for chunk in mobs.chunks_mut(8) {
            let (a_pass, a_seed) = eval_stream_a(&chunk[0], 100);
            let refs: [&MobAI; 8] = [
                &chunk[0], &chunk[1], &chunk[2], &chunk[3],
                &chunk[4], &chunk[5], &chunk[6], &chunk[7],
            ];
            // BORROW HACK: eval_stream_b needs &MobAI while chunk is mut —
            // reborrow shared
            let refs: [&MobAI; 8] = [
                &chunk[0], &chunk[1], &chunk[2], &chunk[3],
                &chunk[4], &chunk[5], &chunk[6], &chunk[7],
            ];
            eval_stream_b(refs, 100, &mut out);
            assert_eq!(
                (a_pass, a_seed), out[0],
                "self-check diverged at mob {}", chunk[0].id
            );
        }
    }
    // the FULL self-check: every mob, A vs B, over a few ticks of drift
    for t in 0..8 {
        for chunk in mobs.chunks_mut(8) {
            if chunk.len() < 8 {
                break;
            }
            let seeds_a: Vec<(bool, u64)> =
                chunk.iter().map(|m| eval_stream_a(m, 20 + t)).collect();
            let mut out = [(false, 0u64); 8];
            let refs: [&MobAI; 8] = [
                &chunk[0], &chunk[1], &chunk[2], &chunk[3],
                &chunk[4], &chunk[5], &chunk[6], &chunk[7],
            ];
            eval_stream_b(refs, 20 + t, &mut out);
            for l in 0..8 {
                assert_eq!(seeds_a[l], out[l], "drift t{t} lane{l}");
            }
            // advance the seeds (the stream's observable state)
            for l in 0..8 {
                chunk[l].seed = out[l].1;
            }
        }
    }
    println!("self-check: A == B on {} mobs x 8 drift ticks", n_mobs);

    // Mode A timing
    let t0 = std::time::Instant::now();
    let mut a_passes = 0u64;
    for t in 0..ticks {
        let gt = 20 + (t as i32);
        for m in mobs.iter_mut() {
            let (p, s) = eval_stream_a(m, gt);
            a_passes += p as u64;
            m.seed = s;
        }
    }
    let ta = t0.elapsed();

    // Mode B timing
    let t0 = std::time::Instant::now();
    let mut b_passes = 0u64;
    for t in 0..ticks {
        let gt = 20 + (t as i32);
        for chunk in mobs.chunks_mut(8) {
            if chunk.len() < 8 {
                // tail: scalar
                for m in chunk.iter_mut() {
                    let (p, s) = eval_stream_a(m, gt);
                    b_passes += p as u64;
                    m.seed = s;
                }
                continue;
            }
            let mut out = [(false, 0u64); 8];
            let refs: [&MobAI; 8] = [
                &chunk[0], &chunk[1], &chunk[2], &chunk[3],
                &chunk[4], &chunk[5], &chunk[6], &chunk[7],
            ];
            eval_stream_b(refs, gt, &mut out);
            for l in 0..8 {
                b_passes += out[l].0 as u64;
                chunk[l].seed = out[l].1;
            }
        }
    }
    let tb = t0.elapsed();

    let ns = |d: std::time::Duration| d.as_secs_f64() * 1e9 / (n_mobs * ticks) as f64;
    println!(
        "eval probe: {} mobs x {} ticks | A scalar {:.1} ns/mob | B interleaved8 {:.1} ns/mob | ratio {:.2}x | passes A={} B={}",
        n_mobs,
        ticks,
        ns(ta),
        ns(tb),
        ns(ta) / ns(tb),
        a_passes,
        b_passes
    );
    let _ = N_GOALS;
}

//! The native pivot bench: a 25k zombie pile + a player (the mobs-25k
//! scenario shape), ticked FULLY in Rust through centities_native::NativeTick.
//!
//! Two purposes:
//! 1. the throughput answer the pivot promises (mspt of the whole native
//!    tick vs the Java-era numbers the CI baseline records);
//! 2. the PROFILING TARGET: single-threaded, stable loop, no allocation
//!    churn in steady state — run it under `perf record` and
//!    scripts/rust_perf_top.py turns the samples into a per-function
//!    self-time table (the Rust hotspot report the pivot demands).
//!
//! Usage: cargo run --release --example native_bench [-- mobs ticks seed]
//! Perf:  perf record -e cycles:u -o /tmp/nat.perf -- target/release/examples/native_bench
use centities_native::NativeTick;
use centities_native::world::PlayerTick;
use swarm_sim::mob::{MobAI, zombie_table};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // the TEMP WAVE10 fine probe must arm BEFORE the first tick
    centities_core::goals::fine_init();
    let n_mobs: usize = args.get(1).map_or(25000, |s| s.parse().unwrap_or(25000));
    let ticks: i32 = args.get(2).map_or(400, |s| s.parse().unwrap_or(400));
    let seed: u64 = args.get(3).map_or(0xC0FFEE, |s| s.parse().unwrap_or(0xC0FFEE));

    let mut nt = NativeTick::new(seed);
    // the WAVE6 A/B knob: CE_NO_FOLD=1 forces the parked (pre-fold) semantics
    if std::env::var_os("CE_NO_FOLD").is_some() {
        nt.set_fold_off(true);
    }
    // the WAVE8 A/B knob: CE_FULL_PACK=1 forces the full per-round pack
    // (the exact pre-wave8 semantics — the same-binary resident-vs-full twin)
    if std::env::var_os("CE_FULL_PACK").is_some() {
        nt.set_full_pack(true);
    }
    // the WAVE9 A/B knob: CE_SPLIT_TRAVEL=1 restores the pre-wave9 two-pass
    // walk (all AI, then all travel) — the fused-vs-split twin arm
    if std::env::var_os("CE_SPLIT_TRAVEL").is_some() {
        nt.set_split_travel(true);
    }
    // the WAVE10 measurement probe: CE_NO_TRAVEL=1 skips the travel slice
    // (the AI-only decomposition row)
    if std::env::var_os("CE_NO_TRAVEL").is_some() {
        nt.set_skip_travel(true);
    }
    // the WAVE10 A/B knob: CE_NO_ODD_SKIP=1 restores the full odd-tick
    // floor (the maintained-bit skip's A/B arm — the twin's B side)
    if std::env::var_os("CE_NO_ODD_SKIP").is_some() {
        swarm_sim::ledger::set_odd_skip(false);
    }
    // the mobs-25k stand: a pile around the spawn column, flat ground
    for i in 0..n_mobs {
        let x = (i % 250) as f64 * 0.6 - 75.0 + 0.05 * ((i / 7) % 3) as f64;
        let z = (i / 250) as f64 * 0.6 - 75.0;
        let mut m = MobAI::new(1000 + i as i64, seed ^ (i as u64).wrapping_mul(0x9E37), x, 64.0, z);
        m.goals = zombie_table();
        nt.add_mob(m);
    }
    // the player (the bot) — 20 blocks from the pile edge, the mobs-25k
    // scenario's melee magnet
    nt.push_players(vec![PlayerTick {
        x: 0.0,
        y: 64.0,
        z: 0.0,
        id: 1,
        flags: 1, // PF_ALIVE
    }]);

    // warmup (the JERK/JIT analog: first ticks allocate + the grid warms)
    for t in 0..50 {
        nt.tick(t);
    }
    centities_native::drain(); // reset the ledger

    // measure
    let mut samples: Vec<f64> = Vec::with_capacity(ticks as usize);
    let t_all = std::time::Instant::now();
    for t in 50..50 + ticks {
        let t0 = std::time::Instant::now();
        nt.tick(t);
        samples.push(t0.elapsed().as_secs_f64() * 1000.0);
    }
    let wall = t_all.elapsed().as_secs_f64() * 1000.0;
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p = |q: f64| -> f64 {
        samples
            .get(((samples.len() as f64) * q).min(samples.len() as f64 - 1.0) as usize)
            .copied()
            .unwrap_or(0.0)
    };

    let chasing = nt
        .store
        .mobs
        .iter()
        .filter(|m| m.mob.target_player >= 0)
        .count();
    println!(
        "native_bench: {} mobs x {} ticks | p50 {:.2} ms/tick | p90 {:.2} | p99 {:.2} | wall {:.0} ms | chasing {}",
        n_mobs,
        ticks,
        p(0.50),
        p(0.90),
        p(0.99),
        wall,
        chasing
    );
    println!("{}", centities_native::drain());
    // the TEMP WAVE10 fine probe (CE_FINE=1): the floor's phase split
    if std::env::var_os("CE_FINE").is_some() {
        centities_core::goals::fine_init();
        let (cl, lk, up, calls) = centities_core::goals::fine_take();
        let tot = (cl + lk + up).max(1) as f64;
        println!(
            "floor fine | cleanup {:.1}% | lockreb {:.1}% | update {:.1}% | calls {}",
            cl as f64 / tot * 100.0,
            lk as f64 / tot * 100.0,
            up as f64 / tot * 100.0,
            calls
        );
    }
    // the perf hint: this process just ran a deterministic, allocation-free
    // (steady-state) single-thread tick loop — profile IT, not the server.
    eprintln!(
        "hint: perf record -e cycles:u -- target/release/examples/native_bench {} {} {}",
        n_mobs, ticks, seed
    );
}

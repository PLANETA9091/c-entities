//! SCALE BENCH — the honest 10^9 question, measured (not extrapolated from
//! vibes). Builds a giant zombie population in the exact index the module
//! serves Paper queries from, then measures the three numbers that decide
//! whether a tick survives:
//!
//!   1. bytes/entity (RAM) — the hard wall for 999,999,999
//!   2. upsert throughput  — spawn/despawn/move pressure per tick
//!   3. query throughput   — the per-tick push/merge/targeting lookups
//!
//! Run: cargo run --release -p centities-core --example scale_bench -- [--n 15000000] [--dense]

use centities_core::{Aabb, EntityIndex};
use std::time::Instant;

fn rss_kb() -> usize {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmRSS"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64 — deterministic, no deps
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn f64in(&mut self, a: f64, b: f64) -> f64 {
        a + (self.next() >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0) * (b - a)
    }
}

fn main() {
    let mut n: usize = 15_000_000;
    let mut dense = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--n" => n = args.next().and_then(|v| v.parse().ok()).unwrap_or(n),
            "--dense" => dense = true,
            _ => {}
        }
    }

    println!(
        "scale_bench: n={} mode={} (vanilla parity scale is 2.5k; this is {}x)",
        n,
        if dense { "dense" } else { "spread" },
        n / 2500
    );

    let mut idx = EntityIndex::new();
    let mut rng = Rng(0x5EED_1);
    let zomb = Aabb::new(0.0, 0.0, 0.0, 0.6, 1.95, 0.6); // vanilla zombie box

    // ---- 1. UPSERT: spawn the whole population -------------------------
    let rss0 = rss_kb();
    let t0 = Instant::now();
    // spread: 3-block lattice over sqrt(n)*3 span; dense: pile into
    // sqrt(n)/8 span so each 64x64 cell holds ~hundreds of bodies
    let span = (n as f64).sqrt() * if dense { 0.4 } else { 3.0 };
    for id in 0..(n as i64) {
        let x = rng.f64in(0.0, span);
        let z = rng.f64in(0.0, span);
        let b = Aabb::new(x, 64.0, z, x + 0.6, 65.95, z + 0.6);
        idx.upsert(id, 0, &b);
    }
    let up_dt = t0.elapsed().as_secs_f64();
    let _ = zomb; // (layout documented; population uses per-entity boxes)
    let rss1 = rss_kb();
    let bpe = (rss1.saturating_sub(rss0)) * 1024 / n;
    println!(
        "upsert: {:>9.0} ents/s ({} in {:.2}s)   RSS delta {} MB -> {:>3} B/entity",
        n as f64 / up_dt, n, up_dt, rss1 / 1024, bpe
    );

    // ---- 2. QUERY: the per-tick lookup pressure ------------------------
    // The module serves, per mob per tick: push scan (getPushableEntities),
    // item merge scans, targeting. A push box is the mob AABB grown by
    // ~1 block; a targeting query is the follow-range (~16-35 blocks).
    let mut qboxes: Vec<Aabb> = Vec::with_capacity(200_000);
    for _ in 0..200_000 {
        let x = rng.f64in(0.0, span);
        let z = rng.f64in(0.0, span);
        qboxes.push(Aabb::new(x, 60.0, z, x + 2.6, 67.0, z + 2.6)); // push-size
    }
    let mut out: Vec<i64> = Vec::with_capacity(4096);
    let mut sink = 0usize;
    // warmup
    for b in &qboxes[..10_000] {
        idx.query(0, b, &mut out);
        sink += out.len() as usize;
    }
    let t1 = Instant::now();
    for b in &qboxes {
        idx.query(0, b, &mut out);
        sink = sink.wrapping_add(out.len() as usize);
    }
    let q_dt = t1.elapsed().as_secs_f64();
    let ns_per_q = q_dt * 1e9 / 200_000.0;
    let hits = sink as f64 / 200_000.0;
    println!(
        "query(push-box 2.6^3): {:>7.0} ns/query ({:.0} q/s/core, avg hits {:.2})",
        ns_per_q,
        1e9 / ns_per_q,
        hits
    );

    // targeting-size query (follow range 16 -> 33x33 box)
    let mut big_hits = 0usize;
    let t2 = Instant::now();
    for b in qboxes.iter().take(50_000) {
        let big = Aabb::new(b.x0 - 14.0, 60.0, b.z0 - 14.0, b.x1 + 14.0, 67.0, b.z1 + 14.0);
        idx.query(0, &big, &mut out);
        big_hits += out.len() as usize;
    }
    let q2_dt = t2.elapsed().as_secs_f64();
    let ns_big = q2_dt * 1e9 / 50_000.0;
    println!(
        "query(target 30^3):    {:>7.0} ns/query ({:.0} q/s/core, avg hits {:.1})",
        ns_big,
        1e9 / ns_big,
        big_hits as f64 / 50_000.0
    );

    // ---- 3. THE 20-TPS VERDICT -----------------------------------------
    // Per tick the mob population generates (honest lower bound): 1 push
    // query per mob (vanilla getPushableEntities runs per LivingEntity),
    // merges for items, targeting pulls at the randomInterval gates.
    let per_tick_push = ns_per_q * (n as f64);
    let push_cores = per_tick_push / 50_000_000.0; // 50 ms budget
    let bpe_1e9 = bpe as f64;
    println!("--- 20 TPS verdict for n={} ---", n);
    println!(
        "push-only per tick: {:>6.1} ms single-core -> {:.1} cores to stay in 50 ms",
        per_tick_push / 1e6, push_cores
    );
    println!(
        "RAM projection: {:.1} B/entity -> 1e9 zombies = {:.0} GB (the index wall; server RAM decides, not CPU)",
        bpe_1e9, bpe_1e9 * 1e9 / 1e9 * 1.0
    );
    println!(
        "DRAM stream floor @1e9*20tps: {:.0} GB/s ({:.0} B/ent * 2e10 upd/s) vs server 8ch DDR5 ~250 GB/s",
        bpe_1e9 * 2e10 / 1e9, bpe_1e9
    );
}

//! ISO bench: split the query into its phases (walk / walk+order / full)
//! to attribute the dense-pile and spread per-query cost. Mirrors the
//! workload of query_bench.rs exactly.
//!
//!   walk      = cellmap probes + len over the visited range
//!   walk+ord  = walk + ensure_cell_order version check
//!   full      = the real query (scan included)   [from query_bench]
//!
//! so scan cost = full - walk+ord, order cost = walk+ord - walk.

use centities_core::{Aabb, EntityIndex};
use std::time::Instant;

fn time_fn<F: FnMut() -> usize>(label: &str, mut f: F, nq: usize) -> (f64, f64) {
    // warmup
    let mut sink = 0usize;
    for b in 0..2000 {
        sink = sink.wrapping_add(f().wrapping_add(b));
    }
    let mut ret_sum = 0usize;
    let dt = Instant::now();
    for _ in 0..nq {
        ret_sum = ret_sum.wrapping_add(f());
    }
    let el = dt.elapsed();
    let per = el.as_nanos() as f64 / nq as f64;
    println!(
        "  {label:<12} {per:>7.1} ns/op  (sink {sink:04x}, ret_avg {:.2})",
        ret_sum as f64 / nq as f64
    );
    (per, ret_sum as f64 / nq as f64)
}

fn run_iso(label: &str, idx: &mut EntityIndex, qboxes: &[Aabb], nq: usize) {
    println!("{label}:");
    let mut i = 0usize;
    time_fn("walk", || {
        i = (i + 1) % qboxes.len();
        idx.iso_walk_only(0, &qboxes[i])
    }, nq);
    time_fn("walk+ord", || {
        i = (i + 1) % qboxes.len();
        idx.iso_no_scan(0, &qboxes[i])
    }, nq);
    let mut out: Vec<i64> = Vec::with_capacity(4096);
    time_fn("full", || {
        i = (i + 1) % qboxes.len();
        idx.query(0, &qboxes[i], &mut out);
        out.len()
    }, nq);
}

fn main() {
    // ---- workload 1: dense pile (identical to query_bench) ----
    let mut idx = EntityIndex::new();
    let mut id: i64 = 1;
    let mut rng = 0x243F6A8885A308D3u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    for pile in 0..64u32 {
        let px = ((pile % 8) as f64) * 200.0 + 100.0;
        let pz = ((pile / 8) as f64) * 200.0 + 100.0;
        for _ in 0..4000 {
            let x = px + (next() % 240) as f64 / 10.0 - 12.0;
            let z = pz + (next() % 240) as f64 / 10.0 - 12.0;
            let y = 64.0 + (next() % 3) as f64;
            let b = Aabb::new(x - 0.125, y, z - 0.125, x + 0.125, y + 0.25, z + 0.125);
            let _ = idx.upsert(id, 0, &b);
            id += 1;
        }
    }
    let mut qboxes: Vec<Aabb> = Vec::new();
    for _ in 0..25000 {
        let (x, z) = if qboxes.len() % 2 == 0 {
            let p = (next() % 64) as u32;
            let px = ((p % 8) as f64) * 200.0 + 100.0;
            let pz = ((p / 8) as f64) * 200.0 + 100.0;
            (
                px + (next() % 240) as f64 / 10.0 - 12.0,
                pz + (next() % 240) as f64 / 10.0 - 12.0,
            )
        } else {
            (
                (next() % 160000) as f64 / 10.0 + 20.0,
                (next() % 160000) as f64 / 10.0 + 20.0,
            )
        };
        let y = 64.0;
        let b = Aabb::new(x - 0.3, y, z - 0.3, x + 0.3, y + 1.8, z + 0.3);
        let _ = idx.upsert(id, 0, &b);
        qboxes.push(b);
        id += 1;
    }
    run_iso("dense-pile", &mut idx, &qboxes, 100_000);

    // ---- workload 2: spread (identical to query_bench) ----
    let mut idx2 = EntityIndex::new();
    let mut id2: i64 = 1;
    for _ in 0..100000 {
        let x = (next() % 640000) as f64 / 10.0;
        let z = (next() % 640000) as f64 / 10.0;
        let y = 64.0 + (next() % 3) as f64;
        let b = Aabb::new(x - 0.125, y, z - 0.125, x + 0.125, y + 0.25, z + 0.25);
        let _ = idx2.upsert(id2, 0, &b);
        id2 += 1;
    }
    let mut qb2: Vec<Aabb> = Vec::new();
    for _ in 0..25000 {
        let x = (next() % 640000) as f64 / 10.0;
        let z = (next() % 640000) as f64 / 10.0;
        let b = Aabb::new(x - 0.3, 64.0, z - 0.3, x + 0.3, 65.8, z + 0.3);
        qb2.push(b);
    }
    run_iso("spread", &mut idx2, &qb2, 100_000);
}

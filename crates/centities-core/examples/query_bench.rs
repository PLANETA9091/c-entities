//! Local micro-benchmark for the exact query scan (doc 21 baseline).
//! Rebuilds the mixed-150k-like workload: dense item piles + mob-sized
//! query boxes on the ground level. Reports ns/query for the raw
//! EntityIndex::query — the native-side cost Java's fast EMA sees.
//!
//! Usage: cargo run --release -p centities-core --example query_bench

use centities_core::{Aabb, EntityIndex};
use std::time::Instant;

fn bench(label: &str, idx: &EntityIndex, boxes: &[Aabb], iters: usize) {
    let mut out: Vec<i64> = Vec::with_capacity(4096);
    let mut sink = 0usize;
    // warmup
    for b in boxes.iter().take(256) {
        idx.query(0, b, &mut out);
        sink += out.len();
    }
    let t0 = Instant::now();
    for _ in 0..iters {
        for b in boxes {
            idx.query(0, b, &mut out);
            sink += out.len();
        }
    }
    let dt = t0.elapsed();
    let nq = iters * boxes.len();
    println!(
        "{label}: {:.0} ns/query ({} queries, {} hits, sink={})",
        dt.as_nanos() as f64 / nq as f64,
        nq,
        out.len(),
        sink & 1
    );
}

fn main() {
    // ---- workload 1: dense pile (mixed-150k style) ----
    // 120k items stacked in piles over a 160x160 block area (all Y ~ground),
    // plus 25k mobs walking the same area. Mob query box ~0.6x1.8x0.6
    // expanded by max_half_span -> scans the whole 16x16 cell it lands in.
    let mut idx = EntityIndex::new();
    let mut id: i64 = 1;
    let mut rng = 0x243F6A8885A308D3u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    // piles: 64 piles of ~4000 items each within a +-12 block footprint
    // (doc 20: mixed fast EMA 11us ~= 1500-2500 candidates/cell)
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
    // mobs — half roaming the open area, half INSIDE piles (the push-query
    // reality of mixed: mobs walk into item piles and push-scan them)
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
    println!(
        "index: {} entities, {} cells, max_half_span={:.3}",
        idx.stats().entities,
        idx.stats().cells,
        idx.max_half_span()
    );
    bench("dense-pile mob query", &idx, &qboxes, 4);

    // ---- workload 2: spread (isolated items, same boxes) ----
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
    println!(
        "index: {} entities, {} cells",
        idx2.stats().entities,
        idx2.stats().cells
    );
    bench("spread mob query", &idx2, &qb2, 4);

    // candidate-count probe for context
    let mut out = Vec::new();
    idx.query(0, &qboxes[0], &mut out);
    println!("sample dense query candidates: {}", out.len());
}

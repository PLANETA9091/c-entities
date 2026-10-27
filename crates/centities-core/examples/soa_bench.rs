//! L5 micro-bench: dense-cell scan throughput (contiguous SoA).
use centities_core::{Aabb, EntityIndex};
use std::time::Instant;

fn main() {
    let mut idx = EntityIndex::new();
    // Dense cell: 700 entities inside one 16x16 cell (mobs-25k shape).
    for i in 0..700u32 {
        let x = (i % 26) as f64 * 0.6;
        let z = (i / 26) as f64 * 0.6;
        idx.upsert(
            1000 + i as i64,
            0,
            &Aabb::new(x, 64.0, z, x + 0.6, 65.8, z + 0.6),
        );
    }
    let q = Aabb::new(7.0, 64.0, 7.0, 8.0, 66.0, 8.0);
    let mut out = Vec::new();
    // warmup
    for _ in 0..2000 {
        idx.query(0, &q, &mut out);
    }
    let t = Instant::now();
    let n = 200_000u32;
    for _ in 0..n {
        idx.query(0, &q, &mut out);
    }
    let dt = t.elapsed();
    println!(
        "dense query (700-entry cell): {:?}/query ({} queries, {} hits avg)",
        dt / n,
        n,
        out.len()
    );
    idx.check_invariants();
    println!("invariants ok");
}

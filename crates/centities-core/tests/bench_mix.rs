//! Micro-benchmark at benchmark-scenario densities (run with --release):
//!   cargo test -p centities-core --release --test bench_mix -- --nocapture --ignored
//!
//! Simulates the mixed-150k workload shape: ~150k entities over a 300x300
//! area (items / zombies / armor stands), one 600x600 tile... no tiles here,
//! just entity queries: zombie push boxes (2x2), item merge boxes (1.5x1.5),
//! and wide sensor boxes (9x9), plus the steady-state same-cell upsert storm.

use centities_core::{Aabb, EntityIndex};
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn f64in(&mut self, a: f64, b: f64) -> f64 {
        a + (self.next() >> 11) as f64 / (1u64 << 53) as f64 * (b - a)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
#[ignore]
fn bench_mix_density() {
    let mut rng = Rng(0x1234_5678_9abc_def0);
    let mut idx = EntityIndex::new();
    let n = 150_000u64;
    let mut entities: Vec<(i64, Aabb)> = Vec::with_capacity(n as usize);

    // --- fill: 50% items, 33% zombies, 17% stands in 300x300 ---
    let t0 = Instant::now();
    for i in 0..n {
        let x = rng.f64in(-150.0, 150.0);
        let z = rng.f64in(-150.0, 150.0);
        let pick = rng.below(100);
        let (hw, hh) = match pick {
            0..=49 => (0.125, 0.25),   // item
            50..=82 => (0.3, 0.9),     // zombie
            _ => (0.25, 0.9875),       // armor stand
        };
        let y = 64.0;
        let b = Aabb::new(x - hw, y, z - hw, x + hw, y + 2.0 * hh, z + hw);
        idx.upsert(i as i64 + 1, 0, &b);
        entities.push((i as i64 + 1, b));
    }
    let fill = t0.elapsed().as_secs_f64();
    let st = idx.stats();
    println!("fill 150k: {fill:.3}s ({:.0} ns/ent), cells={}, ents={}",
        fill * 1e9 / n as f64, st.cells, st.entities);

    // --- query storm: 33k push (2x2), 33k merge (1.5), 33k sensor (9x9) ---
    let mut out: Vec<i64> = Vec::new();
    let mut hits: usize = 0;
    let qn = 33_333;
    let t1 = Instant::now();
    for _ in 0..qn {
        let x = rng.f64in(-150.0, 150.0);
        let z = rng.f64in(-150.0, 150.0);
        idx.query(0, &Aabb::new(x - 1.0, 64.0, z - 1.0, x + 1.0, 66.0, z + 1.0), &mut out);
        hits += out.len();
    }
    for _ in 0..qn {
        let x = rng.f64in(-150.0, 150.0);
        let z = rng.f64in(-150.0, 150.0);
        idx.query(0, &Aabb::new(x - 0.75, 64.0, z - 0.75, x + 0.75, 65.0, z + 0.75), &mut out);
        hits += out.len();
    }
    for _ in 0..qn {
        let x = rng.f64in(-150.0, 150.0);
        let z = rng.f64in(-150.0, 150.0);
        idx.query(0, &Aabb::new(x - 4.5, 60.0, z - 4.5, x + 4.5, 70.0, z + 4.5), &mut out);
        hits += out.len();
    }
    let q = t1.elapsed().as_secs_f64();
    let total = (3 * qn) as f64;
    println!("queries 100k: {q:.3}s ({:.0} ns/query), hits/query={:.1}",
        q * 1e9 / total, hits as f64 / total);

    // --- correctness spot check against brute force (sampled, BEFORE movement) ---
    let qs: Vec<(f64, f64)> = (0..2_000)
        .map(|_| (rng.f64in(-150.0, 150.0), rng.f64in(-150.0, 150.0)))
        .collect();
    let mut miss = 0usize;
    for (x, z) in &qs {
        let q = Aabb::new(x - 2.0, 60.0, z - 2.0, x + 2.0, 70.0, z + 2.0);
        idx.query(0, &q, &mut out);
        let mut want = 0usize;
        for (_, b) in &entities {
            if b.intersects(&q) {
                want += 1;
            }
        }
        if out.len() != want {
            miss += 1;
            if miss == 1 {
                let mut want_ids: Vec<i64> = entities
                    .iter()
                    .filter(|(_, b)| b.intersects(&q))
                    .map(|(id, _)| *id)
                    .collect();
                want_ids.sort();
                let mut got: Vec<i64> = out.clone();
                got.sort();
                println!("MISMATCH q={q:?}");
                println!("  only-in-index: {:?}", got.iter().filter(|i| !want_ids.contains(i)).take(5).collect::<Vec<_>>());
                println!("  only-in-brute: {:?}", want_ids.iter().filter(|i| !got.contains(i)).take(5).collect::<Vec<_>>());
                for (id, b) in &entities {
                    if got.contains(id) != want_ids.contains(id) {
                        println!("  ent {id} box={b:?}");
                    }
                }
            }
        }
    }
    // Contract: false positives within MARGIN are allowed (the Java shim
    // re-checks exactly); false negatives are forbidden. Only-in-brute is a
    // real miss and fails; only-in-index at sub-MARGIN distances is fine.
    let mut fn_misses = 0usize;
    let mut fp_total = 0usize;
    for (x, z) in &qs {
        let q = Aabb::new(x - 2.0, 60.0, z - 2.0, x + 2.0, 70.0, z + 2.0);
        idx.query(0, &q, &mut out);
        let got: Vec<i64> = out.clone();
        for (id, b) in &entities {
            let inb = b.intersects(&q);
            let ing = got.contains(id);
            if inb && !ing {
                fn_misses += 1;
            }
            if !inb && ing {
                fp_total += 1;
            }
        }
    }
    assert_eq!(fn_misses, 0, "FALSE NEGATIVES: {fn_misses}");
    println!("brute-force check: no misses; margin FPs={fp_total}/{} queries", qs.len());

    // --- movement storm: same-cell upserts (jitter every entity) ---
    let t2 = Instant::now();
    for (id, b) in entities.iter_mut() {
        let nx = b.center_x() + rng.f64in(-0.05, 0.05);
        let nz = b.center_z() + rng.f64in(-0.05, 0.05);
        let hw = (b.x1 - b.x0) * 0.5;
        idx.upsert(*id, 0, &Aabb::new(nx - hw, b.y0, nz - hw, nx + hw, b.y1, nz + hw));
        *b = Aabb::new(nx - hw, b.y0, nz - hw, nx + hw, b.y1, nz + hw);
    }
    let mv = t2.elapsed().as_secs_f64();
    println!("move 150k: {mv:.3}s ({:.0} ns/upsert)", mv * 1e9 / n as f64);

    // --- post-move brute force re-check (now with synced reference) ---
    let mut fn_misses2 = 0usize;
    for (x, z) in &qs {
        let q = Aabb::new(x - 2.0, 60.0, z - 2.0, x + 2.0, 70.0, z + 2.0);
        idx.query(0, &q, &mut out);
        let got: Vec<i64> = out.clone();
        for (id, b) in &entities {
            if b.intersects(&q) && !got.contains(id) {
                fn_misses2 += 1;
            }
        }
    }
    assert_eq!(fn_misses2, 0, "post-move FALSE NEGATIVES: {fn_misses2}");
    println!("post-move brute-force check: no misses");
}

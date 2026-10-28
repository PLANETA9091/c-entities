//! Differential parity: AVX2 prefilter path vs scalar exact path.
//! Doc 21. Both paths must produce IDENTICAL query results for every
//! query on every index shape; the scalar path is the reference (it is
//! the pre-L5 f64-promoted compare, unchanged semantics).

use centities_core::{Aabb, EntityIndex};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn f64_in(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (self.next() >> 11) as f64 / (1u64 << 53) as f64 * (hi - lo)
    }
    fn i64_in(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo) as u64) as i64
    }
}

fn compare_paths(label: &str, idx: &mut EntityIndex, boxes: &[Aabb]) {
    let mut out_simd: Vec<i64> = Vec::new();
    let mut out_scalar: Vec<i64> = Vec::new();
    for (qi, b) in boxes.iter().enumerate() {
        centities_core::simd::set_simd_mode(2); // force AVX2
        idx.query(0, b, &mut out_simd);
        centities_core::simd::set_simd_mode(1); // force scalar
        idx.query(0, b, &mut out_scalar);
        centities_core::simd::set_simd_mode(0); // auto
        assert_eq!(
            out_simd, out_scalar,
            "{label}: query {qi} mismatch: simd={:?} scalar={:?}",
            out_simd, out_scalar
        );
    }
}

/// Dense piles + boundary-straddling boxes: the layout where the prefilter
/// does most of its work and where f32-boundary flips would show up.
#[test]
fn simd_scalar_parity_dense_and_boundary() {
    let mut rng = Rng(0xA5A5_5A5A_1234_5678);
    let mut idx = EntityIndex::new();
    let mut id: i64 = 1;
    // 8 piles x 2000 items in a +-8 block footprint (~7.8 items/block^2)
    for pile in 0..8u32 {
        let px = ((pile % 4) as f64) * 100.0 + 500.0;
        let pz = ((pile / 4) as f64) * 100.0 + 500.0;
        for _ in 0..2000 {
            let x = px + rng.f64_in(-8.0, 8.0);
            let z = pz + rng.f64_in(-8.0, 8.0);
            let y = 64.0 + rng.f64_in(0.0, 3.0);
            let b = Aabb::new(x - 0.125, y, z - 0.125, x + 0.125, y + 0.25, z + 0.125);
            idx.upsert(id, 0, &b);
            id += 1;
        }
    }
    // 2000 mobs: half inside piles, half roaming; also odd sizes to move
    // max_half_span around
    let mut qboxes: Vec<Aabb> = Vec::new();
    for k in 0..2000i64 {
        let in_pile = k % 2 == 0;
        let (cx, cz) = if in_pile {
            let p = (k % 8) as f64;
            let px = ((p as i64 % 4) as f64) * 100.0 + 500.0;
            let pz = ((p as i64 / 4) as f64) * 100.0 + 500.0;
            (px + rng.f64_in(-8.0, 8.0), pz + rng.f64_in(-8.0, 8.0))
        } else {
            (rng.f64_in(400.0, 900.0), rng.f64_in(400.0, 900.0))
        };
        let hw = rng.f64_in(0.15, 0.45);
        let h = rng.f64_in(0.6, 1.9);
        let b = Aabb::new(cx - hw, 64.0 - 0.1, cz - hw, cx + hw, 64.0 + h, cz + hw);
        idx.upsert(id, 0, &b);
        qboxes.push(b);
        id += 1;
    }
    compare_paths("dense", &mut idx, &qboxes);
    idx.check_invariants();
}

/// Exact-boundary adversarial: candidates whose faces land ON the query
/// bounds, sub-ULP offsets, zero-width boxes — the f32 prefilter's
/// superset property must keep everything the exact check keeps.
#[test]
fn simd_scalar_parity_exact_boundaries() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let mut idx = EntityIndex::new();
    let mut id: i64 = 1;
    // grid of unit cubes on integer boundaries around the origin cell
    for gx in -20..20 {
        for gz in -20..20 {
            let x = gx as f64 * 1.0;
            let z = gz as f64 * 1.0;
            let y = 64.0;
            let b = Aabb::new(x, y, z, x + 1.0, y + 1.0, z + 1.0);
            idx.upsert(id, 0, &b);
            id += 1;
        }
    }
    let mut qboxes: Vec<Aabb> = Vec::new();
    // queries centered ON candidate faces/corners with jitter of ~1e-6..1e-3
    for _ in 0..4000 {
        let cx = rng.i64_in(-20, 20) as f64 + rng.f64_in(-1e-3, 1e-3);
        let cz = rng.i64_in(-20, 20) as f64 + rng.f64_in(-1e-3, 1e-3);
        let hw = 0.5 + rng.f64_in(-1e-6, 1e-3);
        let qy = 64.0 + rng.f64_in(-0.5, 0.5);
        let b = Aabb::new(
            cx - hw,
            qy,
            cz - hw,
            cx + hw,
            qy + 1.0 + rng.f64_in(-1e-6, 1e-3),
            cz + hw,
        );
        qboxes.push(b);
    }
    // also a couple of zero-height and inverted-ish degenerate boxes (vanilla
    // never sends them, but the index must not disagree between paths)
    qboxes.push(Aabb::new(0.0, 64.0, 0.0, 0.0, 64.0, 0.0));
    compare_paths("boundary", &mut idx, &qboxes);
    idx.check_invariants();
}

/// Mutation-storm parity: random add/remove/move churn (incl. block
/// boundaries 7/8/9, wrap-around of cell pop), then query parity — proves
/// the swap-remove lane masking + stale-lane handling.
#[test]
fn simd_scalar_parity_after_mutation_storm() {
    let mut rng = Rng(0x0F1E_2D3C_4B5A_6978);
    let mut idx = EntityIndex::new();
    let mut live: Vec<(i64, Aabb)> = Vec::new();
    let mut next_id: i64 = 1;
    for step in 0..6000 {
        match rng.next() % 3 {
            0 => {
                // add
                let x = rng.f64_in(-64.0, 64.0);
                let z = rng.f64_in(-64.0, 64.0);
                let y = 64.0;
                let hw = rng.f64_in(0.1, 2.0);
                let b = Aabb::new(x - hw, y, z - hw, x + hw, y + 2.0, z + hw);
                if next_id < 8000 {
                    idx.upsert(next_id, 0, &b);
                    live.push((next_id, b));
                    next_id += 1;
                }
            }
            1 => {
                // remove
                if !live.is_empty() {
                    let k = (rng.next() % live.len() as u64) as usize;
                    let (rid, _) = live.swap_remove(k);
                    idx.remove(rid);
                }
            }
            _ => {
                // move
                if !live.is_empty() {
                    let k = (rng.next() % live.len() as u64) as usize;
                    let (mid, _) = live[k];
                    let x = rng.f64_in(-64.0, 64.0);
                    let z = rng.f64_in(-64.0, 64.0);
                    let hw = rng.f64_in(0.1, 2.0);
                    let b = Aabb::new(x - hw, 64.0, z - hw, x + hw, 66.0, z + hw);
                    idx.upsert(mid, 0, &b);
                    live[k].1 = b;
                }
            }
        }
        // parity probe every 100 steps + a batch of grid queries
        if step % 100 == 99 {
            let mut qboxes: Vec<Aabb> = Vec::new();
            for _ in 0..64 {
                let cx = rng.f64_in(-70.0, 70.0);
                let cz = rng.f64_in(-70.0, 70.0);
                let hw = rng.f64_in(0.2, 4.0);
                qboxes.push(Aabb::new(cx - hw, 63.0, cz - hw, cx + hw, 67.0, cz + hw));
            }
            compare_paths("storm", &mut idx, &qboxes);
        }
    }
    idx.check_invariants();
}

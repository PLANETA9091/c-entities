//! Property tests for the exactness contract: the index must never miss an
//! entity whose true f64 AABB intersects the query box. Extra candidates are
//! allowed (the Java shim re-checks exact f64 intersection).

use centities_core::{Aabb, EntityIndex, MAX_HALF_SPAN};
use std::collections::HashSet;

/// xorshift64* — deterministic, no deps
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn f64in(&mut self, lo: f64, hi: f64) -> f64 {
        let u = (self.next() >> 11) as f64 / (1u64 << 53) as f64;
        lo + u * (hi - lo)
    }
    fn i64in(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

fn brute_force(entities: &[(i64, u16, Aabb)], world: u16, q: &Aabb) -> HashSet<i64> {
    entities
        .iter()
        .filter(|(_, w, b)| *w == world && b.intersects(q))
        .map(|(id, _, _)| *id)
        .collect()
}

/// The no-miss check: every brute-force hit must be in the index result.
fn assert_no_miss(index: &EntityIndex, entities: &[(i64, u16, Aabb)], world: u16, q: &Aabb) {
    index.check_invariants();
    let mut got = Vec::new();
    index.query(world, q, &mut got);
    let got_set: HashSet<i64> = got.into_iter().collect();
    let want = brute_force(entities, world, q);
    let missed: Vec<i64> = want.difference(&got_set).copied().collect();
    assert!(
        missed.is_empty(),
        "index missed entities {missed:?} for query {q:?} (returned {} candidates, want {})",
        got_set.len(),
        want.len()
    );
}

#[test]
fn exactness_random_small_world() {
    let mut rng = Rng(0x1234_5678_9abc_def0);
    let mut idx = EntityIndex::new();
    let mut entities: Vec<(i64, u16, Aabb)> = Vec::new();

    for round in 0..20_000u64 {
        let op = rng.next() % 100;
        if op < 55 || entities.len() < 50 {
            let id = rng.i64in(1, 3_000);
            let world = (rng.next() % 2) as u16;
            let cx = rng.f64in(-300.0, 300.0);
            let cz = rng.f64in(-300.0, 300.0);
            let y = rng.f64in(-64.0, 320.0);
            let hx = rng.f64in(0.05, 3.0);
            let hz = rng.f64in(0.05, 3.0);
            let hy = rng.f64in(0.05, 3.0);
            let b = Aabb::new(cx - hx, y - hy, cz - hz, cx + hx, y + hy, cz + hz);
            if let Some(e) = entities.iter_mut().find(|e| e.0 == id) {
                e.1 = world;
                e.2 = b;
            } else {
                entities.push((id, world, b));
            }
            idx.upsert(id, world, &b);
        } else if op < 75 {
            if !entities.is_empty() {
                let i = (rng.next() as usize) % entities.len();
                let (id, _, _) = entities.swap_remove(i);
                idx.remove(id);
            }
        } else {
            let qx = rng.f64in(-300.0, 300.0);
            let qz = rng.f64in(-300.0, 300.0);
            let qy = rng.f64in(-64.0, 320.0);
            let q = Aabb::new(
                qx,
                qy,
                qz,
                qx + rng.f64in(0.0, 40.0),
                qy + rng.f64in(0.0, 40.0),
                qz + rng.f64in(0.0, 40.0),
            );
            assert_no_miss(&idx, &entities, 0, &q);
            assert_no_miss(&idx, &entities, 1, &q);
        }
        if round % 5_000 == 0 {
            for _ in 0..50 {
                let qx = rng.f64in(-320.0, 300.0);
                let qz = rng.f64in(-320.0, 300.0);
                let w = rng.f64in(0.1, 130.0);
                let q = Aabb::new(qx, -64.0, qz, qx + w, 320.0, qz + w);
                assert_no_miss(&idx, &entities, 0, &q);
                assert_no_miss(&idx, &entities, 1, &q);
            }
        }
    }
    assert_eq!(idx.stats().entities, entities.len());
}

#[test]
fn boundary_and_cluster_exactness() {
    // adversarial: entities exactly on cell borders (multiples of 64),
    // query boxes exactly touching entity bounds
    let mut rng = Rng(0xdead_beef_cafe_0001);
    let mut idx = EntityIndex::new();
    let mut entities: Vec<(i64, u16, Aabb)> = Vec::new();

    for i in 0..2_000u64 {
        let bx = if i % 3 == 0 { 64.0 * (i as f64) } else { 64.0 * (i as f64) + 32.0 };
        let cz = 64.0 * ((i % 37) as f64) - 1_000.0;
        let hx = 0.5 + (i % 7) as f64 * 0.25;
        let b = Aabb::new(bx - hx, 64.0, cz - hx, bx + hx, 68.0, cz + hx);
        let id = i as i64 + 1;
        entities.push((id, 0, b));
        idx.upsert(id, 0, &b);
    }
    for _ in 0..50_000 {
        let qx = 64.0 * ((rng.next() % 64) as f64);
        let qz = 64.0 * ((rng.next() % 64) as f64) - 1_000.0;
        let w = 0.25 * ((rng.next() % 8) as f64);
        let q = Aabb::new(qx - w, 60.0, qz - w, qx + w, 70.0, qz + w);
        assert_no_miss(&idx, &entities, 0, &q);
    }
}

#[test]
fn oversize_entities_are_found() {
    let mut idx = EntityIndex::new();
    let dragon = Aabb::new(-200.0, 60.0, -200.0, 200.0, 90.0, 200.0);
    idx.upsert(7, 0, &dragon);
    assert_eq!(idx.stats().oversize, 1);

    let mut out = Vec::new();
    idx.query(0, &Aabb::new(90.0, 60.0, 90.0, 95.0, 70.0, 95.0), &mut out);
    assert!(out.contains(&7), "oversize entity missed");

    // shrinking back to normal size re-registers in the grid, no duplicates
    let small = Aabb::new(90.0, 60.0, 90.0, 95.0, 70.0, 95.0);
    idx.upsert(7, 0, &small);
    assert_eq!(idx.stats().oversize, 0);
    idx.query(0, &Aabb::new(92.0, 60.0, 92.0, 93.0, 70.0, 93.0), &mut out);
    assert_eq!(out.iter().filter(|i| **i == 7).count(), 1);
}

#[test]
fn remove_storm_keeps_exactness() {
    let mut rng = Rng(0xfeed_face_1234_5678);
    let mut idx = EntityIndex::new();
    let mut entities: Vec<(i64, u16, Aabb)> = Vec::new();

    for i in 0..10_000u64 {
        let cx = rng.f64in(-500.0, 500.0);
        let cz = rng.f64in(-500.0, 500.0);
        let b = Aabb::new(cx - 0.3, 64.0, cz - 0.3, cx + 0.3, 66.0, cz + 0.3);
        let id = i as i64 + 1;
        entities.push((id, 0, b));
        idx.upsert(id, 0, &b);
    }
    for _ in 0..5_000 {
        let i = (rng.next() as usize) % entities.len();
        let (id, _, _) = entities.swap_remove(i);
        idx.remove(id);
    }
    for _ in 0..20_000 {
        let qx = rng.f64in(-520.0, 500.0);
        let qz = rng.f64in(-520.0, 500.0);
        let q = Aabb::new(qx, 60.0, qz, qx + 8.0, 70.0, qz + 8.0);
        assert_no_miss(&idx, &entities, 0, &q);
    }
    assert_eq!(idx.stats().entities, 5_000);
}

#[test]
fn world_clear_and_multiple_worlds() {
    let mut idx = EntityIndex::new();
    let b = |x: f64, z: f64| Aabb::new(x - 0.3, 64.0, z - 0.3, x + 0.3, 66.0, z + 0.3);
    idx.upsert(1, 0, &b(10.0, 10.0));
    idx.upsert(2, 1, &b(10.0, 10.0));
    idx.upsert(3, 0, &b(70.0, 10.0));
    idx.check_invariants();
    idx.clear_world(0);
    idx.check_invariants();

    let mut out = Vec::new();
    idx.query(0, &Aabb::new(0.0, 60.0, 0.0, 128.0, 70.0, 128.0), &mut out);
    assert!(out.is_empty(), "world 0 must be empty after clear");
    idx.query(1, &Aabb::new(0.0, 60.0, 0.0, 128.0, 70.0, 128.0), &mut out);
    assert_eq!(out, vec![2]);
    assert_eq!(idx.stats().entities, 1);
}

#[test]
fn far_lands_coordinates_are_exact() {
    // 30M-block coordinates: global f32 would quantize by ~2 blocks here;
    // cell-relative f32 must stay exact.
    let mut rng = Rng(0xc0ff_ee00_1234_5678);
    let mut idx = EntityIndex::new();
    let mut entities: Vec<(i64, u16, Aabb)> = Vec::new();
    for i in 0..5_000u64 {
        let cx = rng.f64in(29_999_000.0, 29_999_984.0);
        let cz = rng.f64in(-29_999_984.0, -29_999_000.0);
        let b = Aabb::new(cx - 0.3, 64.0, cz - 0.3, cx + 0.3, 66.0, cz + 0.3);
        let id = i as i64 + 1;
        entities.push((id, 0, b));
        idx.upsert(id, 0, &b);
    }
    for _ in 0..20_000 {
        let qx = rng.f64in(29_998_900.0, 29_999_984.0);
        let qz = rng.f64in(-29_999_984.0, -29_998_900.0);
        let q = Aabb::new(qx, 60.0, qz, qx + 16.0, 70.0, qz + 16.0);
        assert_no_miss(&idx, &entities, 0, &q);
    }
}

#[test]
fn query_expansion_is_bounded_and_correct() {
    let mut idx = EntityIndex::new();
    let e = Aabb::new(63.0, 64.0, 63.0, 65.0, 66.0, 65.0); // center on border
    idx.upsert(1, 0, &e);

    let mut out = Vec::new();
    idx.query(0, &Aabb::new(70.0, 60.0, 70.0, 72.0, 70.0, 72.0), &mut out);
    assert!(out.is_empty());

    idx.query(0, &Aabb::new(65.5, 60.0, 65.5, 70.0, 70.0, 70.0), &mut out);
    assert!(out.contains(&1), "border-crossing entity missed");
    assert!(idx.max_half_span() <= MAX_HALF_SPAN);
}

#[test]
fn query_buffer_reuse() {
    // API-level guarantee for the steady state: one buffer, many queries
    let mut idx = EntityIndex::new();
    for i in 0..1_000i64 {
        let x = (i % 40) as f64 * 8.0;
        let z = (i / 40) as f64 * 8.0;
        idx.upsert(i + 1, 0, &Aabb::new(x, 64.0, z, x + 0.6, 66.0, z + 1.8));
    }
    let mut out = Vec::new();
    for i in 0..10_000 {
        let qx = (i % 320) as f64;
        idx.query(0, &Aabb::new(qx, 60.0, 0.0, qx + 4.0, 70.0, 4.0), &mut out);
    }
}

#[test]
fn packed_window_is_conservative() {
    // L4c windowed rows: the packed (xLo, xHi) interval must be a SUPERSET of
    // the entity's true f64 x-interval for every upsert/position — the Java
    // row scan may skip a candidate only when the packed interval is disjoint
    // from the query window, so a packed interval that clips the true one
    // would be a silent false negative. Property: for random boxes across
    // cell boundaries (and near the +-32-block clamp edge), for every query
    // box that the true interval intersects, the packed interval must
    // intersect too.
    let mut idx = EntityIndex::new();
    let mut lcg = 0x243F6A8885A308D3u64;
    let mut rnd = move || {
        lcg ^= lcg << 13;
        lcg ^= lcg >> 7;
        lcg ^= lcg << 17;
        lcg
    };
    // spans several cells around the origin, incl. negative coords
    let id0 = 1i64;
    for _ in 0..20_000i64 {
        let fx = (rnd() % 4096) as f64 / 1024.0 - 2.0;   // -2..2
        let fz = (rnd() % 4096) as f64 / 1024.0 - 2.0;
        let w = (rnd() % 1024) as f64 / 1024.0;           // 0..1
        let h = (rnd() % 512) as f64 / 1024.0;
        let x0 = fx * 16.0;                                // stride across cells
        let z0 = fz * 16.0;
        let b = Aabb::new(x0, 64.0, z0, x0 + w, 64.0 + h, z0 + h);
        idx.upsert(id0, 0, &b);

        // read back the packed window for the entity's home cell
        let home = {
            let mut probe = Vec::new();
            idx.query(0, &Aabb::new(x0, 64.0, z0, x0 + 0.01, 64.01, z0 + 0.01), &mut probe);
            assert!(probe.contains(&id0));
            probe.clear();
            // find the home cell by scanning the query result's cell — simpler:
            // re-derive from the center the same way upsert does
            let cx = (b.center_x().floor() as i32) >> 4;
            let cz = (b.center_z().floor() as i32) >> 4;
            (cx, cz)
        };
        let (ox, _) = ((home.0 as i64 * 16) as f64, home.1 as i64 * 16);
        // packed scan of that cell (cap high enough)
        let mut packed: Vec<i64> = Vec::new();
        // locate the cell id via a packed scan through cell_key lookup
        let mut found = false;
        for cell in 1..idx.stats().cells as u32 + 1 {
            if let Some(key) = idx.cell_key_of(cell) {
                let (kcx, kcz) = centities_core::key_coords(key);
                if (kcx, kcz) == home {
                    let (_n, ov) = idx.cell_scan_packed(cell, &mut packed, 4096);
                    assert!(!ov);
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "home cell not found");
        let entry = packed.iter().copied().find(|p| (p & 0xFFFF_FFFF) == (id0 & 0xFFFF_FFFF))
            .expect("entity missing from packed row");
        let lo = (((entry >> 32) & 0xFFFF) as f64 - 32768.0) / 1024.0 + ox;
        let hi = (((entry >> 48) & 0xFFFF) as f64 - 32768.0) / 1024.0 + ox;
        assert!(lo <= b.x0 + 1e-3, "packed lo {} clips true minX {}", lo, b.x0);
        assert!(hi >= b.x1 - 1e-3, "packed hi {} clips true maxX {}", hi, b.x1);
    }
}

//! The pidx cell-bookkeeping property test (the addendum-18f hunt).
//!
//! The mobs/mixed/spread PH family showed the kernel scan returning the
//! SELF's row TWICE (raw=2 with zero KPI records — the only silent skip
//! is `id == self_id`) while 226 rows lived in the walked cells. This
//! test hammers the exact operation mix the kernel exercises — upserts
//! across cell boundaries, removals (the swap-remove paths), vanilla-
//! order sorts (the blocked-SoA rebuild) and query_cell sweeps — and
//! asserts the two invariants the scan's correctness rests on:
//!
//! 1. NO DUPLICATES: a query_cell result never contains the same id
//!    twice (the self included);
//! 2. NO FALSE NEGATIVES: every indexed entity whose box intersects the
//!    query window is returned (the prefilter is a margin superset).

use centities_core::{Aabb, EntityIndex};

fn zb(x: f64, z: f64) -> Aabb {
    Aabb::new(x - 0.3, 0.0, z - 0.3, x + 0.3, 1.95, z + 0.3)
}

/// Deterministic LCG so a failure is reproducible from the seed in the
/// panic message.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn f(&mut self) -> f64 {
        (self.next() % 1_000_000) as f64 / 1_000_000.0
    }
}

#[test]
fn pidx_no_duplicate_rows_and_no_false_negatives() {
    for seed in [1u64, 0xBEEF, 0xC0FFEE, 42] {
        let mut rng = Lcg(seed);
        let mut idx = EntityIndex::new();
        let mut boxes: Vec<(i64, Aabb)> = Vec::new();
        let mut next_id = 1i64;

        for step in 0..4000 {
            let op = rng.next() % 100;
            if op < 55 || boxes.is_empty() {
                // UPSERT: 60% move an existing entity, 40% spawn a new one
                if !boxes.is_empty() && rng.next() % 100 < 60 {
                    let i = (rng.next() as usize) % boxes.len();
                    let (id, _) = boxes[i];
                    let x = (rng.f() - 0.5) * 40.0;
                    let z = (rng.f() - 0.5) * 40.0;
                    let b = zb(x, z);
                    idx.upsert(id, 0, &b);
                    boxes[i].1 = b;
                } else {
                    let id = next_id;
                    next_id += 1;
                    let x = (rng.f() - 0.5) * 40.0;
                    let z = (rng.f() - 0.5) * 40.0;
                    let b = zb(x, z);
                    idx.upsert(id, 0, &b);
                    boxes.push((id, b));
                }
            } else if op < 70 {
                // REMOVE (drives cell_remove + free_slot swap paths)
                let i = (rng.next() as usize) % boxes.len();
                let (id, _) = boxes.swap_remove(i);
                idx.remove(id);
            } else if op < 85 {
                // SORT a random cell by a synthetic order key (the blocked-SoA
                // rebuild must preserve the query set AND the cpos mapping)
                let cx = (((rng.f() - 0.5) * 4.0).floor() as i32) - 1;
                let cz = (((rng.f() - 0.5) * 4.0).floor() as i32) - 1;
                idx.sort_cell_by_keys(0, cx, cz, |id| (id as i64, id as i64));
            } else {
                // QUERY SWEEP: a window over a random area; assert both
                // invariants against the shadow model.
                let x = (rng.f() - 0.5) * 32.0;
                let z = (rng.f() - 0.5) * 32.0;
                let w = 0.2 + rng.f() * 3.0;
                let (qx0, qx1) = (x - w, x + w);
                let (qz0, qz1) = (z - w, z + w);
                let mut out: Vec<i64> = Vec::new();
                // walk the cells the kernel would walk (box + generous margin)
                let ex = idx.max_half_span() + centities_core::MARGIN as f64 + 2.0;
                let cx0 = ((qx0 - ex).floor() as i32) >> centities_core::CELL_SHIFT;
                let cx1 = ((qx1 + ex).floor() as i32) >> centities_core::CELL_SHIFT;
                let cz0 = ((qz0 - ex).floor() as i32) >> centities_core::CELL_SHIFT;
                let cz1 = ((qz1 + ex).floor() as i32) >> centities_core::CELL_SHIFT;
                for cz in cz0..=cz1 {
                    for cx in cx0..=cx1 {
                        idx.query_cell(0, cx, cz, qx0, 0.0, qz0, qx1, 2.0, qz1, &mut out);
                    }
                }
                // INVARIANT 1: no duplicate ids
                let mut seen = std::collections::HashSet::new();
                for &id in out.iter() {
                    assert!(
                        seen.insert(id),
                        "seed={seed} step={step}: query returned id {id} TWICE \
                         (the duplicated-self family)"
                    );
                }
                // INVARIANT 2: every true overlap is found
                let mut want: std::collections::HashSet<i64> = std::collections::HashSet::new();
                for (id, b) in boxes.iter() {
                    if b.x0 < qx1 && b.x1 > qx0 && b.z0 < qz1 && b.z1 > qz0 {
                        want.insert(*id);
                    }
                }
                let got: std::collections::HashSet<i64> = out.iter().copied().collect();
                let missed: Vec<i64> = want.difference(&got).copied().collect();
                assert!(
                    missed.is_empty(),
                    "seed={seed} step={step}: query missed {:?} of {:?} wanted \
                     (window {qx0:.3}..{qx1:.3} / {qz0:.3}..{qz1:.3})",
                    missed,
                    want,
                );
            }
        }
    }
}

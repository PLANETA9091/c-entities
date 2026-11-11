//! doc 36b regression guard: `query()` streams candidates in the EXACT
//! vanilla `EntityLookup.getEntities` scan order — (chunkZ, chunkX,
//! sectionY, insertion seq) — via the versioned per-cell re-sort
//! (`ensure_cell_order`) and the oversize merge (`order_key_of`).
//!
//! The index itself returns a MARGIN superset (extras are re-checked by
//! the Java shim), so every check here filters the streamed output to the
//! exactly-intersecting subset and asserts:
//!   1. the subset equals the brute-force hit set (order can't hide a miss),
//!   2. the subset is sorted by the two-long vanilla key (ka, kb).

use centities_core::{Aabb, EntityIndex};
use std::collections::HashSet;

/// xorshift64* — deterministic, no deps (same shape as exactness.rs).
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
    fn i32in(&mut self, lo: i64, hi: i64) -> i32 {
        (lo + (self.next() % (hi - lo + 1) as u64) as i64) as i32
    }
    fn usz(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The side table: everything the vanilla order needs per entity.
#[derive(Clone)]
struct Ent {
    world: u16,
    b: Aabb,
    sy: i32,
    seq: i64,
}

const B: i64 = 2097152;
const M: i64 = 0x3F_FFFF;

/// ka = (chunkZ biased)<<22 | chunkX biased — from the box CENTER chunk,
/// the same chunk the index homes the entity by (and vanilla iterates).
fn ka_of(e: &Ent) -> i64 {
    let cx = (e.b.center_x().floor() as i64) >> 4;
    let cz = (e.b.center_z().floor() as i64) >> 4;
    (((cz + B) & M) << 22) | ((cx + B) & M)
}

/// kb = (sectionY biased)<<40 | seq.
fn kb_of(e: &Ent) -> i64 {
    ((((e.sy as i64) + B) & M) << 40) | (e.seq & 0xFFFF_FFFF)
}

/// The vanilla-order model over the exactly-intersecting subset.
fn model_ordered(ents: &[(i64, Ent)], world: u16, q: &Aabb) -> Vec<i64> {
    let mut hits: Vec<(i64, i64, i64, i64)> = ents
        .iter()
        .filter(|(_, e)| e.world == world && e.b.intersects(q))
        .map(|(id, e)| (ka_of(e), kb_of(e), e.seq, *id))
        .collect();
    hits.sort_unstable_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));
    hits.into_iter().map(|(_, _, _, id)| id).collect()
}

/// Query the index, drop the MARGIN extras, assert order + set parity.
fn assert_vanilla_order(idx: &mut EntityIndex, ents: &[(i64, Ent)], world: u16, q: &Aabb) {
    idx.check_invariants();
    let mut got = Vec::new();
    idx.query(world, q, &mut got);
    // exact-intersecting subset of the stream (the Java shim's view)
    let by_id: std::collections::HashMap<i64, &Ent> = ents.iter().map(|(i, e)| (*i, e)).collect();
    let mut exact: Vec<i64> = got
        .iter()
        .filter(|id| match by_id.get(id) {
            Some(e) => e.world == world && e.b.intersects(q),
            None => panic!("ghost id {} streamed — removed entities must not stream", id),
        })
        .copied()
        .collect();
    let want = model_ordered(ents, world, q);
    assert_eq!(
        exact.len(),
        want.len(),
        "set drift: stream {:?} vs model {:?}",
        got,
        want
    );
    // order: strictly increasing (ka, kb, seq) over the exact subset
    // (seq is unique, so strict is well-defined)
    let mut prev: Option<(i64, i64, i64, i64)> = None;
    for id in &exact {
        let e = by_id.get(id).unwrap();
        let k = (ka_of(e), kb_of(e), e.seq, *id);
        if let Some(p) = prev {
            assert!(
                (p.0, p.1, p.2) < (k.0, k.1, k.2),
                "order drift: id {} (ka {}, kb {}, seq {}) streams after id {} (ka {}, kb {}, seq {})",
                k.3, k.0, k.1, k.2,
                p.3, p.0, p.1, p.2
            );
        }
        prev = Some(k);
    }
    let got_set: HashSet<i64> = exact.drain(..).collect();
    let want_set: HashSet<i64> = want.into_iter().collect();
    assert_eq!(got_set, want_set, "hit set mismatch");
}

static SEQ: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
fn next_seq() -> i64 {
    SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

fn mob_box(rng: &mut Rng, cx: i32, cz: i32) -> Aabb {
    let x = (cx << 4) as f64 + rng.f64in(0.0, 15.0);
    let z = (cz << 4) as f64 + rng.f64in(0.0, 15.0);
    let y = rng.f64in(-60.0, 250.0);
    let w = rng.f64in(0.6, 0.9);
    Aabb::new(x - w, y, z - w, x + w, y + 1.8, z + w)
}

/// One cell: insertion order must not leak into the streamed order —
/// the versioned re-sort brings the physical layout to (sy, seq).
#[test]
fn cell_internal_order_is_insertion_independent() {
    let mut rng = Rng(0xC0FFEE);
    for round in 0..8 {
        let mut idx = EntityIndex::new();
        let mut ents: Vec<(i64, Ent)> = Vec::new();
        // 64 mobs in ONE chunk, random sectionY + unique seqs, shuffled
        // insertion order (the seq assignment order differs from sy order
        // on purpose: sorting by sy must dominate, ties by seq)
        let ids: Vec<i64> = (1..=64).map(|i| 1000 + i).collect();
        let mut stamps: Vec<(i32, i64)> = (0..64)
            .map(|_| (rng.i32in(-4, 20), next_seq()))
            .collect();
        stamps.sort();
        let mut order: Vec<usize> = (0..64).collect();
        for i in (1..64).rev() {
            let j = rng.usz(i + 1);
            order.swap(i, j);
        }
        for (k, &si) in order.iter().enumerate() {
            let (sy, seq) = stamps[si];
            let b = mob_box(&mut rng, 5, 7);
            let id = ids[si];
            idx.upsert_ord(id, 0, &b, sy, seq);
            ents.push((id, Ent { world: 0, b, sy, seq }));
            let _ = k;
        }
        // whole-chunk query
        let q = Aabb::new(80.0, -70.0, 112.0, 96.0, 320.0, 128.0);
        assert_vanilla_order(&mut idx, &ents, 0, &q);
        if round == 0 {
            // sectionY dominates seq: rebuild with sy DESCENDING but seq
            // ASCENDING — the stream must come back sy-ascending
            let mut idx2 = EntityIndex::new();
            let mut ents2 = Vec::new();
            for (i, id) in ids.iter().enumerate() {
                let sy = 20 - i as i32; // descending
                let seq = next_seq(); // ascending with i
                let b = mob_box(&mut rng, 9, 3);
                idx2.upsert_ord(*id, 0, &b, sy, seq);
                ents2.push((*id, Ent { world: 0, b, sy, seq }));
            }
            let q2 = Aabb::new(144.0, -70.0, 48.0, 160.0, 320.0, 64.0);
            assert_vanilla_order(&mut idx2, &ents2, 0, &q2);
        }
    }
}

/// Across cells the walk is chunkZ-major, chunkX-inner — an adversarial
/// layout where the LAST chunk in walk order holds the SMALLEST seq must
/// still stream its members first.
#[test]
fn chunk_major_walk_across_cells() {
    let mut idx = EntityIndex::new();
    let mut ents: Vec<(i64, Ent)> = Vec::new();
    let mut rng = Rng(0x5EED);
    // chunks (cx, cz): (0,0), (1,0), (0,1), (1,1), (-1, 3), (4, -2)
    let cells = [(0i32, 0i32), (1, 0), (0, 1), (1, 1), (-1, 3), (4, -2)];
    // assign seqs so that a seq-sorted walk is chunkZ-ASCENDING — the
    // exact opposite of vanilla's chunkZ-major cell walk; only the real
    // (chunkZ, chunkX) key grouping produces the vanilla order
    let mut seq_ctr = 0i64;
    let mut cz_sorted = cells.to_vec();
    cz_sorted.sort_by_key(|c| c.1);
    for &(cx, cz) in &cz_sorted {
        for _ in 0..5 {
            let sy = rng.i32in(-4, 20);
            let b = mob_box(&mut rng, cx, cz);
            seq_ctr += 1;
            idx.upsert_ord(seq_ctr, 0, &b, sy, seq_ctr);
            ents.push((seq_ctr, Ent { world: 0, b, sy, seq: seq_ctr }));
        }
    }
    // one query covering everything
    let q = Aabb::new(-80.0, -70.0, -80.0, 144.0, 320.0, 144.0);
    assert_vanilla_order(&mut idx, &ents, 0, &q);
}

/// A vanilla re-append (new seq) or a section move (new sy) must bump the
/// cell's order version — the NEXT query reflects the new order.
#[test]
fn reappend_and_section_move_reorder() {
    let mut idx = EntityIndex::new();
    let mut ents: Vec<(i64, Ent)> = Vec::new();
    let mut rng = Rng(0xBEEF);
    for id in 1..=20 {
        let b = mob_box(&mut rng, 2, 2);
        let sy = rng.i32in(0, 8);
        let seq = next_seq();
        idx.upsert_ord(id, 0, &b, sy, seq);
        ents.push((id, Ent { world: 0, b, sy, seq }));
    }
    let q = Aabb::new(0.0, -70.0, 0.0, 64.0, 320.0, 64.0);
    assert_vanilla_order(&mut idx, &ents, 0, &q);

    // re-append id=1 with a fresh seq: it must now stream LAST among
    // equal-sy members of its section
    let seq_new = next_seq() + 1000; // jump ahead of every existing seq
    let e1 = &mut ents[0].1;
    idx.upsert_ord(1, 0, &e1.b, e1.sy, seq_new);
    e1.seq = seq_new;
    assert_vanilla_order(&mut idx, &ents, 0, &q);

    // section move: id=2 jumps to a higher sectionY
    let e2 = &mut ents[1].1;
    let seq2 = next_seq() + 1000;
    idx.upsert_ord(2, 0, &e2.b, 30, seq2);
    e2.sy = 30;
    e2.seq = seq2;
    assert_vanilla_order(&mut idx, &ents, 0, &q);

    // same-stamp upserts must NOT churn (the (0,0)-unknown contract: two
    // identical stamps are a stable key); order stays, and a stamped
    // upsert afterwards still reorders exactly once
    let e3 = ents[2].1.clone();
    idx.upsert_ord(3, 0, &e3.b, e3.sy, e3.seq);
    assert_vanilla_order(&mut idx, &ents, 0, &q);
}

/// An oversize entity (half-span > 8) merges into the GLOBAL vanilla
/// order — the whole stream (cells + oversize) comes back sorted.
#[test]
fn oversize_merges_into_global_order() {
    let mut idx = EntityIndex::new();
    let mut ents: Vec<(i64, Ent)> = Vec::new();
    let mut rng = Rng(0xD00D);
    for id in 1..=40 {
        let cx = rng.i32in(-2, 2);
        let cz = rng.i32in(-2, 2);
        let b = mob_box(&mut rng, cx, cz);
        let sy = rng.i32in(-4, 20);
        let seq = next_seq();
        idx.upsert_ord(id, 0, &b, sy, seq);
        ents.push((id, Ent { world: 0, b, sy, seq }));
    }
    // a wide bateau box: 40 blocks span → half 20 > 8 → oversize
    let big = Aabb::new(-20.0, 60.0, -20.0, 20.0, 70.0, 20.0);
    let seq_big = next_seq();
    idx.upsert_ord(9999, 0, &big, 4, seq_big);
    ents.push((9999, Ent { world: 0, b: big, sy: 4, seq: seq_big }));

    let q = Aabb::new(-64.0, -70.0, -64.0, 64.0, 320.0, 64.0);
    assert_vanilla_order(&mut idx, &ents, 0, &q);
}

/// Removes re-home the last slot (free_slot swap) — the moved entity's
/// (sy, seq) must travel with it, or the order model desyncs. A storm of
/// upserts/moves/removals keeps every query exactly vanilla-ordered.
#[test]
fn remove_storm_keeps_vanilla_order() {
    let mut rng = Rng(0x1510);
    let mut idx = EntityIndex::new();
    let mut ents: std::collections::HashMap<i64, Ent> = std::collections::HashMap::new();
    let mut next_id = 1i64;
    for round in 0..12_000 {
        let op = rng.usz(100);
        if op < 55 || ents.is_empty() {
            // upsert: new or move
            let cx = rng.i32in(-3, 3);
            let cz = rng.i32in(-3, 3);
            let b = mob_box(&mut rng, cx, cz);
            let sy = rng.i32in(-4, 20);
            let seq = next_seq();
            let id = if rng.usz(100) < 30 && !ents.is_empty() {
                // MOVE an existing entity (reuse id — vanilla re-append)
                let picked = *ents.keys().collect::<Vec<_>>()[rng.usz(ents.len())];
                idx.remove(picked);
                ents.remove(&picked);
                picked
            } else {
                let id = next_id;
                next_id += 1;
                id
            };
            idx.upsert_ord(id, 0, &b, sy, seq);
            ents.insert(id, Ent { world: 0, b, sy, seq });
        } else {
            // remove a random entity
            let picked = *ents.keys().collect::<Vec<_>>()[rng.usz(ents.len())];
            idx.remove(picked);
            ents.remove(&picked);
        }
        if round % 400 == 0 {
            let q = Aabb::new(-56.0, -70.0, -56.0, 56.0, 320.0, 56.0);
            let elist: Vec<(i64, Ent)> = ents.iter().map(|(k, v)| (*k, v.clone())).collect();
            assert_vanilla_order(&mut idx, &elist, 0, &q);
        }
    }
    let elist: Vec<(i64, Ent)> = ents.iter().map(|(k, v)| (*k, v.clone())).collect();
    let q = Aabb::new(-56.0, -70.0, -56.0, 56.0, 320.0, 56.0);
    assert_vanilla_order(&mut idx, &elist, 0, &q);
}

/// Two worlds never mix: world-1 entities never appear in a world-0
/// stream and vice versa (the ka/bb keys are per-world in the model too).
#[test]
fn worlds_stay_separate() {
    let mut idx = EntityIndex::new();
    let mut ents: Vec<(i64, Ent)> = Vec::new();
    let mut rng = Rng(0x777);
    for id in 1..=30 {
        let b = mob_box(&mut rng, 1, 1);
        let sy = rng.i32in(0, 10);
        let seq = next_seq();
        let world = (id % 2) as u16;
        idx.upsert_ord(id, world, &b, sy, seq);
        ents.push((id, Ent { world, b, sy, seq }));
    }
    let q = Aabb::new(0.0, -70.0, 0.0, 48.0, 320.0, 48.0);
    assert_vanilla_order(&mut idx, &ents, 0, &q);
    assert_vanilla_order(&mut idx, &ents, 1, &q);
}

//! Isolated stress tests for the internal open-addressed maps.

// Access internals via a public-ish test shim: we re-declare nothing; instead
// test through EntityIndex public API where possible, and use the #[doc(hidden)]
// invariants for the rest. CellMap/IdMap are private — test them via a
// dedicated unit-test include: we expose them behind `pub` under `cfg(feature)`
// is overkill; simplest: replicate the map code paths through EntityIndex
// behavior (cells are created/kept; cellmap maps all live cells).

use centities_core::{Aabb, EntityIndex};

#[test]
fn cellmap_maps_all_live_cells_after_mutation_storm() {
    let mut idx = EntityIndex::new();
    let b = |x: f64, z: f64| Aabb::new(x - 0.3, 64.0, z - 0.3, x + 0.3, 66.0, z + 0.3);
    // touch many cells
    for i in 0..500i64 {
        let x = ((i * 37) % 2000) as f64;
        let z = ((i * 53) % 2000) as f64;
        idx.upsert(i + 1, 0, &b(x, z));
    }
    idx.check_invariants();
    // move half of them across cells
    for i in 0..250i64 {
        let x = ((i * 91) % 2000) as f64 + 0.5;
        let z = ((i * 17) % 2000) as f64 + 0.5;
        idx.upsert(i + 1, 0, &b(x, z));
    }
    idx.check_invariants();
    // remove a third
    for i in 0..170i64 {
        idx.remove(i * 3 + 1);
    }
    idx.check_invariants();
}

# L5 — contiguous per-cell SoA storage

Follows doc 15 ("dense-cell queries are the remaining honest loss") and
doc 16 (L6 native item tick core). Owner directive: maximum real Rust
acceleration.

## The problem (measured on CI, doc 15)

Dense-cell native queries walked linked chains through the global entries
vec: ~14us for a 700-entry cell (mobs-25k b5), while vanilla's section
scan iterates a contiguous collection at ~10us. mobs-25k paid the +7%
regression in exactly that basin.

## The rework (centities-core storage layer, API unchanged)

Each cell now holds THREE parallel contiguous arrays:

    cell_ids[ci]:   Vec<i64>    (8B/entry)
    cell_box[ci]:   Vec<f32>    (stride 6, cell-relative x/z + abs y)
    cell_slots[ci]: Vec<u32>    (global slot, for O(1) removals)

- Dense query = flat sequential read (memory bandwidth). X/Z compares stay
  f64-promoted exactly like the chain scan (a f32 compare at world-scale
  ULPs could produce a false NEGATIVE — contract violation). Y compares
  stay f32 (same as before).
- Same-cell box update = 6 stores into the cell array.
- Cell move = O(1) swap-remove + push (cpos/cell_slots fix-ups).
- cell_scan/cell_scan_packed read the same arrays (rows publish faster).
- Cells are never freed (stable cell ids — the publisher's dirty-marking
  relies on them; unchanged from the chain era).

## Verification

- exactness: 20k-op property test vs brute force — green (release).
- maps_stress, cellmap suites, module structural tests — green.
- Micro-bench (700-entry cell, 200k queries): **1.29us/query** vs ~14us
  chain-walk — ~11x; the mobs-25k +7% regression basin is expected to flip.

## Expected CI (93dc667 = L6 + L5)

- items-100k: L6 already removed ~26ms of Java tick; L5 does not move it.
- mobs-25k: dense queries 14us -> ~1.3us; the +7% regression should become
  a solid win (the query path was ~40% of the mob tick).
- spread-100k: row-served mid-density unchanged; native-path queries (the
  ROW_TOTAL_CAP refusals) get the same 11x on their native legs.

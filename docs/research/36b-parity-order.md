# 36b. The vanilla scan order — the versioned parity-order index

Date: 2026-11-12. Status: SHIPPED (this commit). Basis: the ORDER-CHECK
diagnostics (f118b0c, c21d642) + the fastQuery EXACT AABB re-check
(9f11b3c) caught the last behavioral divergence class: the CANDIDATE
ORDER of shared lookups.

## Why order is behavior

Vanilla `EntityLookup.getEntities` streams candidates in a deterministic
scan order: chunkZ-major, chunkX-inner over the entity's home chunks,
then per chunk by section Y and insertion order. Three vanilla consumers
are ORDER-SENSITIVE, not just SET-sensitive:

- **push / collisions** — the crammed-pair cap (`maxEntityCollisions`)
  keeps the FIRST N candidates in scan order; a different order keeps a
  different subset and the shove differs;
- **merge scans (items)** — which item instance absorbs which is a
  first-match decision over the stream;
- **targeting / sensors** — nearest-pick ties are broken by stream order.

A reordered stream diverged the parity harness legs (the merge-scan
mismatch the ORDER-CHECK log caught on merge scans).

## Design: stamps + versioned lazy re-sort

The vanilla key is two longs (the same shape as CeHooks.ordKey):

- `ka = (chunkZ+2^21 & 0x3FFFFF) << 22 | (chunkX+2^21 & 0x3FFFFF)`
- `kb = (sectionY+2^21 & 0x3FFFFF) << 40 | (seq & 0xFFFFFFFF)`

with `seq` = the global `ORD_SEQ` insertion stamp Java already maintains
at addEntity / moveEntity re-append (CeHooks:1102/:1126).

1. **Java carries the stamps everywhere.** `B_SY`/`B_SEQ` are stamped for
   EVERY entity (items merge-scan through the same index; the stamps were
   previously living-entity-only). `nativeFlush`, `queryNative` and
   `queryNative2` all take `(batchSy, batchSeq)` int arrays; the JNI
   descriptors grew `[I[I` twice (the natives_parity.rs contract test now
   pins them).

2. **The index stores the stamps per slot** (`upsert_ord(id, world, b,
   sy, seq)`; plain `upsert` = (0,0) for tests/pidx views). A changed
   `(sy, seq)` on a same-cell upsert bumps the cell's order version
   (`cell_ov`); a move/remove/add bumps both cells'.

3. **The physical layout re-sorts lazily.** `ensure_cell_order` compares
   `cell_sv` (sorted-to version) with `cell_ov` and physically rebuilds
   the blocked-SoA arrays in `(sy, seq)` order only when they differ —
   once per cell between actual order changes, not once per query.
   `query()` calls it per touched cell before the AVX2 scan streams it;
   `publish` calls `ensure_order(cell)` before a row is copied to Java
   (rowQuery serves candidates in row order).

4. **Oversize merges back into the global order.** The oversize list now
   carries `(sy, seq)` too; when an oversize entity intersects a query,
   the whole stream (cell parts + oversize) is re-sorted by the full
   `(ka, kb)` via `order_key_of` (id → slot → stamps, oversize → record).
   Rare by construction — empty in every bench scenario.

Within one cell chunkZ/chunkX are constant, so the per-cell sort key
reduces to `(sectionY, seq)` — a single-i64 sort.

## Cost

Zero on the hot path when order is stable: one versioned compare per
touched cell (two u64 loads). The re-sort itself is an O(n log n) build
of `(slot, kb)` rows + `write_cell_layout` — amortized once per cell
between order changes; the 12k-op removal storm test keeps it hot and
still benches inside noise.

## The regression gate (tests/order_parity.rs)

Six tests against an independent vanilla-order model
(`(chunkZ, chunkX, sectionY, seq)` over the exactly-intersecting subset —
margin extras are dropped exactly as the Java shim re-checks them):

1. `cell_internal_order_is_insertion_independent` — shuffled insertion,
   64 mobs, one chunk; plus the sy-descending/seq-ascending adversary
   (sectionY must dominate seq).
2. `chunk_major_walk_across_cells` — six chunks, seqs assigned so a
   seq-sorted walk is chunkZ-ASCENDING (the opposite of vanilla); only
   the real key grouping passes.
3. `reappend_and_section_move_reorder` — fresh-seq re-append streams
   last; a sectionY jump reorders; a same-stamp upsert does not churn.
4. `oversize_merges_into_global_order` — a 40-block boat + 40 mobs.
5. `remove_storm_keeps_vanilla_order` — 12k ops; every 400th query is
   order-checked; ghost ids (removed entities still streaming) fail
   loudly; the free_slot last-swap must carry `(sy, seq)` with the moved
   entity or this test desyncs.
6. `worlds_stay_separate` — two worlds over the same coords never mix.

All six passed FIRST RUN on the shipped implementation — the stamp
plumbing was already consistent end to end (flush + query batch + row
publish); the suite pins it from now on.

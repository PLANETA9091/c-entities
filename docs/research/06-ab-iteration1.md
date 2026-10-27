# A/B iteration 1: module v0.1 vs vanilla — results and next moves

Run: GitHub Actions `baseline` @ `075641a` (ubuntu-latest, 4 vCPU, JFR on both sides,
same scenario parameters, module scenario additionally boots CRUSSTY + c-entities
and asserts `ARMED` before measurement — the job fails otherwise).

## Numbers

| scenario | mspt mean | p50 | p99 | heap MB | verdict |
|---|---:|---:|---:|---:|---|
| items-100k | 63.07 | 59.69 | 102.13 | 2217 | vanilla |
| items-100k-mod | 73.02 | 69.75 | 115.50 | 2250 | **+15.8% slower** |
| mixed-150k | 509.71 | 505.84 | 648.83 | 1587 | vanilla |
| mixed-150k-mod | 871.22 | 864.64 | 1108.34 | 1722 | **+70.9% slower** |

The module is armed and functionally correct (all patches `rc=0`, e2e green,
no crashes across 600 measured ticks), but v0.1 **loses** to vanilla in the
current benchmark scenarios. This is an honest measurement, not a failure of
the harness — the harness did exactly its job.

## Why the fast path does not win here

1. **Scenario geometry defeats the XZ grid.** The bench spawns 100–150k
   entities inside a 150×150-block area (~5–7 entities per block!). The Rust
   index uses 64-block XZ cells — the whole population lives in 1–2 cells, so
   `queryNative` returns almost every entity as a candidate. The Java-side
   recheck (`box.intersects` + predicate per candidate) then does the same
   O(N) AABB scan vanilla does — the grid filters nothing.
   Hotspots agree: `AABB.intersects` is 25% of vanilla CPU and **42%** in the
   mod run (more total samples, same work + overhead).
2. **Probes are per-event JNI calls.** `Entity.setPos` is woven with a prefix
   probe — every position update of every entity crosses JNI into Rust and
   takes the global `INDEX` mutex. Items settle/merge every tick in this
   scenario → millions of JNI round-trips per second of pure overhead.
3. **The recheck double-pays.** `fastQuery` resolves candidates via
   `sl.getEntity(id)` and re-tests `box.intersects` — when the candidate set
   is huge this duplicates the vanilla cost on top of the native query.

## What this means for the design

The levers (docs/research/04) are still right — the *real-world* win of a
spatial index comes from entities being spread over hundreds/thousands of
blocks, not packed in a cube. But the module must also not regress in the
pathological case. Iteration 2 targets:

1. **3D candidate filtering in Rust.** Keep the XZ grid for bucketing, add a
   per-cell Y-sorted slab (or per-entity AABB in the index) so `queryNative`
   returns only actual box hits; the Java recheck then collapses to a
   cheap trust check (length equality) instead of a full intersects scan.
2. **Amortized probes.** Replace per-`setPos` JNI with a per-tick bulk sync:
   the index refreshes positions of entities observed ticking this tick
   (single JNI pass over a dirty list), pos probes become no-ops for
   already-synced entities (dirty-flag stamped by the tick sweep).
3. **Sharded index lock** (per-world / per-cell-strip) to remove the global
   mutex from the probe path.
4. **A realistic-distribution scenario** (`spread-100k`: same 100k entities
   over a 1500×1500 area) alongside the dense stress case — the A/B must
   show wins where players actually live, and the dense case guards against
   pathological regressions.

Success criteria for iteration 2 (same pipeline, same scenarios):
- `spread-100k-mod` ≥ 30% faster than `spread-100k` vanilla;
- `items-100k-mod` / `mixed-150k-mod` within ±5% of vanilla (no regression);
- ARM assertion still green in every module run.

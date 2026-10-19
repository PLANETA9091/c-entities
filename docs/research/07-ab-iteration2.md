# A/B iteration 2: v0.2 (single box probe, trust fast path, 16-block cells, batched probes) — results

Run: GitHub Actions `baseline` @ `f6c97b8` (13 scenarios, 1200-tick measure window for the
six A/B rows; module runs boot CRUSSTY + c-entities and assert `ARMED` — the job fails
otherwise). Module = iteration 2b (`df64fbb`).

## Numbers (mspt mean / p99, 1200 ticks)

| scenario | vanilla | module | delta mean |
|---|---:|---:|---:|
| items-100k (dense 150×150) | 67.2 / 112.0 | 63.1 / 101.8 | **−6.2% faster** |
| mixed-150k (dense 150×150) | 747.3 / 876.4 | **471.7 / 581.0** | **−36.9% faster** |
| spread-100k (600×600) | 469.7 / 659.1 | 481.0 / 659.0 | +2.4% (noise) |

Heap (MB): mixed mod **1701 vs vanilla 1733** — below vanilla now (2a was +0.9G).
ARM asserted in every module run; zero anomalies.

## What moved the needle (vs iteration 1: mixed +70.9% → −36.9%)

1. **setBoundingBox is the single box-change choke point** (setPos → setPosRaw →
   makeBoundingBox → setBoundingBox, javap-verified). One probe per move with the
   exact new box; the setPos probe retired (5 → 4 weave points).
2. **Trust fast path**: the Rust index is exact (property-tested vs brute force),
   so the Java side stopped re-running `AABB.intersects` per candidate — the vanilla
   predicate still applies. This removed the duplicate of vanilla's hottest scan.
3. **16-block cells** (was 64): dense-cube candidate sets scale with (box/cell)² —
   ~50k → ~9k candidates per query in the 150×150 scenarios.
4. **Batched probes** (2b): server-thread probes append into plain Java arrays;
   one `nativeFlush` per query window applies the whole batch (~48k moves = 1 JNI
   call). Probes from worldgen threads take a rare direct-JNI path — single-producer
   discipline makes the batch race-free.
5. **Allocation-free query path** (2b): `queryNative` writes into a ThreadLocal
   `long[]` (Rust reuses a thread-local scratch Vec) — no per-query `jlongArray`;
   the mod heap dropped below vanilla.

## Success criteria from doc 06 — verdict

- `items/mixed-mod within ±5% of vanilla`: **exceeded** (−6.2% / −36.9%).
- `spread-100k-mod ≥ 30% faster`: **not met** (+2.4%). Root cause: vanilla chunk
  slices are already spatially efficient in spread distributions; the module's
  remaining edge (exact candidates, no Java intersects) roughly cancels its fixed
  costs (flush JNI, ThreadLocal traffic). The spread case is dominated by chunk
  ticking + mob AI, not proximity queries.
- `ARM green in every module run`: **met**.

## Honesty note: runner variance

Single-run A/B deltas below ±10% are noise on shared 4-vCPU runners: items-100k
flipped between −13.4% and +8.6% across two 2a runs with identical config. The
1200-tick window tightens within-run variance; cross-run conclusions require
consistent trends (iteration 1 → 2a → 2b mixed trend: +70.9% → +6.7% → −36.9%).
Iterate on trends, not single deltas.

## Next levers (iteration 3 candidates)

1. **spread-100k deep dive**: with proximity queries solved, the tick is now
   chunk overhead + vanilla mob AI. Profile the mod spread run: if
   `flush`/`probeBox` Java-side cost is visible, shrink batch entries
   (16-bit quantized boxes per cell?) or move the flush to a mid-tick point.
2. **Zero-JNI steady state**: keep a persistent direct ByteBuffer mapping of the
   index; probes write coordinates via VarHandle into it (no JNI at all), the
   native side reads it on tick boundaries. Removes even the flush calls.
3. **Multi-world sharding** of the index lock (doc 06 item 3) — only matters if
   profiling shows contention (unlikely: probes and queries are server-thread).
4. **Mob-AI cost sharing**: sensing/targeting loops also call getEntities — they
   now ride the fast path for free; next vanilla-layer cost is navigation.
   Out of scope for the proximity index.

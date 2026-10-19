# Iteration 4a: the parity gate — the owner's hard requirement enforced by measurement

Owner directive (2026-10-19): **the module must never be slower than
vanilla**. This closes the open decision at the end of doc 08 — the
per-query boundary must be moved, and until the zero-JNI architecture
lands, nothing may ship that measures slower than the vanilla path it
replaces.

## What shipped (4a)

Three mechanisms, all driven by live measurements, no behavior change to
results (every path returns exactly what the replaced vanilla body
returns):

1. **Parity gate.** `Level.getEntities` is replaced by a router. Per
   (world × query-volume bucket) it keeps an EMA of the measured cost of
   both paths and sends each query to the cheaper one. Bootstrap: the
   first 16 queries per bucket go fast (measured probes); after that a
   1/256 re-probe keeps the EMAs honest, and the comparison runs with
   hysteresis (fast must be < 80% of mirror). Consequence: in steady
   state the module pays `min(fast, mirror)` per bucket — parity with
   vanilla is a property of the architecture, not a hope. Where the fast
   path cannot win (small queries — the fixed JNI+native cost dominates),
   the module IS vanilla.

2. **Suspend/resume.** If a 16384-decision gate window chooses fast fewer
   than 32 times, indexing suspends: probes cost one volatile read, the
   Rust index is dropped (`nativeReset`), the arena is cleared. Any
   sampled mirror query costing ≥ 15 µs re-arms it through a full resync
   (`getAllEntities()` → batch upserts → one flush). A world that never
   benefits from the index therefore converges to exact vanilla cost +
   one volatile read per entity event.

3. **Zero-resolve arena.** Candidates resolve through `ARENA[id & MASK]`
   — entity refs written by the probes themselves (~2 ns), validated by
   an id match, with the id-keyed getter as fallback only. This removes
   the ~190 ns/candidate `LevelEntityGetter.byId` resolve that doc 08
   identified as the dense-case killer. The Rust index stays exact: no
   intersects pass, candidates arrive only if their box truly hits.

Plus two vanilla-compatibility fixes found while wiring the gate:

- **source exclusion.** The fast path never excluded the `source` entity;
  vanilla's `EntityGetter.getEntities` excludes exactly it. A query whose
  only candidate is the source (item-merge self-scan) returned it. Fixed
  by an id skip at resolve time.
- **false desyncs.** The desync guard (`resolved==0 && n>0 → mirror`)
  fired on legitimate cases: all-stale candidates (remove still sitting
  in the batch — index momentarily ahead of the world) and all-source
  candidates. Both are counted separately now (`staleCand`, srcSkip);
  desync again means a genuine index/world disagreement about living
  entities. After the fix, local runs show `desync=0` through 8k-entity
  merge storms (191k box probes over 600 ticks).

Also: the world registry became a `ConcurrentHashMap` (identity
semantics preserved — `Level` inherits `Object.equals`); the old
`IdentityHashMap` could corrupt its buckets when a worldgen thread
registered a level concurrently with the server thread.

## Local sanity (8k items, area 60, 300-tick window, module=on)

- boot: ARMED, all three patches applied, `Done` 15.7 s
- gate: `fast=133 mirror=1799` — merge queries are small, so after
  bootstrap the gate routes them to vanilla, as designed
- `desync=0`, `stale=519` (batching window, correctly not desyncs),
  `arenaMiss=519` (all stale-related), no suspend (window not reached)
- mspt mean: module 9.00 / 9.13 vs vanilla 9.38 on identical configs
  (within noise, consistently not slower)

The honest verdict at 100k+ scale belongs to CI (with player, 1200-tick
windows): the gate's own numbers — which path each bucket chose and at
what measured price — are the acceptance test for "not slower than
vanilla".

## What the gate does NOT fix

The fixed ~8.4 µs per-query JNI+native cost is routed around, not
removed. When the fast path wins a bucket, it wins by `mirror_cost −
fast_cost` — and `fast_cost` still contains the crossing. Iteration 4b
(zero-JNI steady state) attacks the constant itself; the gate is the
safety envelope that makes every intermediate state of that work
shippable: any bucket where a new implementation is not yet faster
automatically falls back to vanilla.

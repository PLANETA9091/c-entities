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

## First CI A/B on the gate (4a.0, run 39293284393, with player, 1200 ticks)

| pair | vanilla | module | delta | fast/mirror (mod) |
|---|---:|---:|---:|---|
| items-100k | 96.95 | 105.52 | +8.9% | 133 / 5 123 |
| spread-100k | 482.5 | 491.4 | +1.9% | 648k / 18.3M |
| mixed-150k | 826.7 | 868.3 | **+5.0%** (was **+64.7%**) | 80k / 20.5M |

- The gate collapsed the mixed-150k regression from +64.7% to +5.0% while
  remaining fully live (`desync=0` everywhere, 19.6M box probes indexed).
- items +8.9% has NO systematic source: CPU profiles of both runs are
  identical (no CeHooks/JNI/GC entry in the module profile; the intersects
  share is even lower), probe+flush arithmetic is < 0.1 ms/tick — runner
  noise (known ±10% job-to-job spread). To be confirmed by a repeat run.
- The real defect is visible in mixed-mod: `fast=80429` with
  `reprobe=80254` — almost every fast query was a 1/256 re-probe, priced
  at ~307 µs (native query cost on a loaded 2-vCPU runner; also the EMA
  was bootstrapped on an empty world during fill). 66 probes/tick ×
  307 µs ≈ 20 ms/tick — almost the whole remaining delta. The 1/8 EMA
  drift is too slow to escape a bad plateau.
- Zero-resolve verdict: `resMs=1.2s` over 4.06M resolved candidates =
  **15 ns/candidate** (was ~210 ns) — the resolve loop is defeated.

## 4a.1 fix: re-probe backoff + EMA unstick

- Per-cell re-probe backoff: when a fast sample lands > 2× the mirror
  price, the probe step doubles (1/256 → 1/4096) until fast gets cheap
  again; a sample below mirror restores the granularity.
- EMA unstick: a sample > 4× the current fast EMA replaces it outright —
  an EMA trained on an empty world must jump, not creep.
- Self-review caught an operator-precedence bug in the first draft of the
  step mask (`256 << min(bk,4) - 1` shifts by bk−1 → shift-by-−1 at bk=0);
  fixed to an explicit `step = 256 << min(bk,4); mask = step − 1`.

## 4a.1 CI (run 39298866543): the storm appears

| pair | vanilla | module | delta | fast/mirror (mod) | batchEntries |
|---|---:|---:|---:|---|---:|
| items-100k | 114.4 | 83.4 | **−27.1%** | 135 / 5 232 | — |
| mixed-150k | 1091.9 | 649.4 | **−40.5%** | 5 174 / 20.0M | 2.0M |
| spread-100k | 479.9 | 581.1 | **+21.1%** | 4 488 / 18.4M | **115.7M** |

- Runner-to-runner spread is huge (mixed vanilla 826.7 → 1091.9 between
  runs; module 868.3 → 649.4) — single-pair deltas are ±30% noise; the
  internal counters are the only trustworthy signal.
- **The +21.1% spread delta is a real defect, precisely identified:**
  `batchEntries = 115.6M` against `16.1M` probes — 65.8k batch entries per
  tick. That is a suspend↔resync cycle: the 4a.1 hard unstick jumped the
  fast EMA to a single 307 µs re-probe sample, the gate went mirror
  forever, `fastChosen` hit 0, the window filled in ~1 tick (17k mirror
  queries/tick), the index suspended — and the very next sampled mirror
  (spread's ~30 µs ≥ the 15 µs resume threshold) resumed it with a full
  100k-entity resync. One resync ≈ 5–8 ms/tick of pure tax, paid every
  tick. Items and mixed — the same build — were fine (−27%/−40%).
- Zero-resolve stays proven: 15 ns/candidate.

## 4a.2: storm guards

- Soft unstick: a sample > 4× EMA moves the EMA half-way toward it, not
  onto it — one expensive re-probe cannot erase a bucket's memory where
  fast genuinely wins most of the time.
- Suspend is suppressed while the last sampled mirror price is ≥
  RESUME_NANOS: a dense world in mirror mode must keep indexing and keep
  measuring, or the gate guarantees itself a resync storm.
- Post-resume cooldown: 4 full gate windows must pass with a live index
  before suspend may fire again.
- RESUME_NANOS 15 → 30 µs (spread's average mirror price must not wake
  the index by itself).
- Local falling-item sanity after the change: `fast=3218 desync=0 ARMED`.

## What the gate does NOT fix

The fixed ~8.4 µs per-query JNI+native cost is routed around, not
removed. When the fast path wins a bucket, it wins by `mirror_cost −
fast_cost` — and `fast_cost` still contains the crossing. Iteration 4b
(zero-JNI steady state) attacks the constant itself; the gate is the
safety envelope that makes every intermediate state of that work
shippable: any bucket where a new implementation is not yet faster
automatically falls back to vanilla.

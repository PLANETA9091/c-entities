# 4b.2 — zero-JNI steady state: design

Owner requirement unchanged: **the module must never be slower than
vanilla**. 4a.3 proved the gate (paired same-runner A/B: mixed −17.2%,
items −1.7%, spread +3.9% inside the 1.05 margin). This doc designs the
iteration that attacks the remaining constant.

## Where the remaining per-query cost lives

A fast query today (post 4b.1):

| phase | cost | note |
|---|---:|---|
| gate: instanceof + worldId memo + bucketOf | ~10-15 ns | memo landed in 4b.1 |
| wantFast (EMA compare / re-probe dice) | ~5 ns | |
| JNI crossing (flush-merged queryNative) | ~150-300 ns | 11 args, 2 arrays, no allocs |
| native index query (Rust grid walk) | ~30-300 ns | candidate-count dependent |
| batch apply inside the crossing | ~20-50 ns/entry | amortized over queries |
| resolve loop (Java): ARENA + id-match + predicate | ~10-20 ns/candidate | |
| ArrayList alloc (vanilla-parity fresh list) | ~40 ns | required by parity |

For dense buckets (mixed-150k) the native walk replaces a
`ChunkEntitySlices` section scan of thousands — fast wins big (−17%).
For sparse buckets (spread-100k) the mirror is already ~0.3-1 µs; the
crossing + fixed overheads eat the win, which is exactly the observed
+3.9%: the fast path is competitive, not winning.

## Goal

Spread-class buckets strictly under parity: total fast-path cost below
the mirror's section-scan cost, WITHOUT giving up the tiny-window
correctness of flush-in-query.

## Non-goals / rejected designs

- **Tick-boundary batch flush** (scheduled task flushes once per tick):
  rejected on correctness. Flush-in-query is what bounds the
  remove-batch window to microseconds; a tick-wide window would return
  entities that vanilla already removed (push/merge predicates do not
  all filter removed entities — the desync guard cannot tell "stale
  index" from "stale world" at that width). Total CPU is also invariant:
  the batch work moves, it does not shrink.

- **ThreadLocal result list reuse**: rejected on parity. Callers of
  `Level.getEntities` legitimately hold the returned list across
  nested queries (push → move → collision → getEntities); clear()-reuse
  silently truncates the outer iteration. Vanilla returns a fresh list;
  so do we.

## Design: Java-visible cell rows, native-published

Keep flush-in-query. Add a Java-side READ-ONLY mirror of the index's
per-cell candidate lists:

- **Layout (Java)**: `long[][] CELL_ROWS` — one row per active cell:
  `[epoch:32][world:16][cellKey:48]…ids…`. Plus `CELL_INDEX`
  (open-addressed int[] keyed on the same hash the Rust grid uses) mapping
  cellKey → row slot.
- **Publish (native, at flush)**: the flush-merged `queryNative` already
  takes the index mutex. After applying the batch, the native marks every
  cell touched since the last publish DIRTY and re-emits ONLY dirty cells'
  id lists into the Java rows (regenerate, not incremental — cells are
  small; dense cells are routed by the gate to the native path anyway).
  A monotonically increasing `publishSeq` stamps the world.
- **Query (Java, steady state)**: compute the covered cell range from the
  box (1-2 cells for push/merge boxes ≤ 64 blocks; larger boxes fall back
  to the native path immediately). If every covered row has `epoch ==
  publishSeq` (or was untouched by this tick's batch — `lastClean >=
  batchStartSeq`), iterate the rows IN JAVA: ARENA resolve, id-match,
  AABB filter (Java-side `intersects`, ~10-15 ns/candidate), predicate.
  **Zero JNI.**
- **Miss → the current native path** (crossing + native query), and the
  same crossing publishes the rows it walks — a miss pays once per dirty
  cell per tick, then the cell is Java-readable for the rest of the tick.
- **Correctness**: rows are regenerated from the authoritative index
  AFTER the batch apply inside the SAME mutex section — a row is either
  pre-batch (epoch old → miss → native) or post-batch exact. Single
  server thread reads rows; probes only append to the batch (never to
  rows). The desync guard stays as the tripwire: any row/native answer
  mismatch beyond threshold flips ACTIVE=false, exactly today.
- **Parity envelope**: the gate gains a third tier per bucket
  (native-fast / java-fast / mirror); EMA prices each tier as today, so a
  java-fast row regime that loses to mirror auto-reverts. No change to
  the suspend/resync logic.

## Expected effect

- spread-100k: ~30-60k queries/tick × ~150-300 ns crossing → ~5-18 ms/tick
  removed once rows cover the working set; the remaining per-candidate
  Java work is what the mirror was already doing with worse data layout.
- mixed-150k: unchanged (dense buckets stay native — that path already
  wins by 17%).
- Risk: the cell-row memory is O(active cells × avg ids/cell) — bounded
  by publishing only cells seen by queries; over-publishing dense cells
  is wasted bandwidth, the gate's dense buckets simply do not read rows.

## Staging

1. **4b.2a** — native publish of dirty rows + Java row table + read-only
   use for SINGLE-cell boxes with `epoch == publishSeq`; every miss falls
   to the current path. Gate unchanged (java-fast still counts as "fast"
   for the EMA — it is priced by the same dt).
2. **4b.2b** — multi-cell coverage + third gate tier + counters
   (`javaFast`, `rowMiss`, `rowStale`) in the hooks@ line.
3. CI: ab-paired decides; spread must go ≤ 0.99, mixed must stay ≤ 1.00,
   items ≤ 1.00. Any regression → gate margin shrinks or tier disabled
   by config flag before revert.

## Immediate next steps (this ladder, in order)

1. 4b.2a native + Java surface (this design, staging above).
2. L2 static fast-path (doc 04) — resting items skip setOldPos/flags/
   effects/sync rescans; items dominate every 100k+ scenario.
3. L3 tile short-circuits — hopper no-op detection.

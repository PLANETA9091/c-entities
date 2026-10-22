# L4c — graded row ceiling + latch hysteresis consistency

Follows doc 14 (L4 query dominance, L4b win latch). Owner mandate unchanged:
outputs byte-identical, never slower than vanilla. Owner directive: real
Rust acceleration, no tick-skipping.

## What CI said about L4b (run 39741539403, commit 6a56dd9)

Ten of eleven baseline legs succeeded (mixed-150k hit the job timeout and
was cancelled; publish skipped). Paired A/B p50:

| scenario | vanilla | module | delta |
|---|---:|---:|---:|
| items-100k | 95.9 | 48.8 | **-49.1%** |
| mobs-25k  | 448.0 | 285.2 | **-36.3%** (first module leg ever) |
| spread-100k | 470.5 | **517.8** | **+10.0% REGRESSION** (was -5.8% at 49182d2) |

## Spread post-mortem (counter + profile evidence)

`spread-100k` = 60k items + 20k armor stands + 20k zombies over AREA=600.
Every living entity runs a push query per tick (`LivingEntity.pushEntities`
-> `getPushableEntities` -> the hooked `Level.getEntities`): ~10.5k
queries/tick, ~11 candidates each.

1. **The flat 64-candidate row ceiling (L3.1) misfired.** spread rows hold
   100-300 candidates (mid-density: dense relative to the ceiling, sparse
   relative to items-100k's ~830/cell). `rowBig=4,020,609` refusals
   (~10.6k/tick) pushed ALL of them onto the JNI native path:
   `fastMs` delta = **208 ms/tick** (~30us per query), visible as +10% p50.
   The JFR profile undercounts this — `jdk.ExecutionSample` only samples at
   safepoints, and a thread inside JNI is not at one; the counters caught
   what the profile hid.
2. **The L4b win latch ignored the hysteresis it was meant to shortcut.**
   mobs-25k w2b0: 3 lucky ~1us re-probes latched the bucket, then every
   query paid the ~13us fast average against a **4us mirror** (fast EMA
   250us had been saying so all along, N=1M). The break condition
   (`dt > 4 x MIRR_EMA` = 16us) fired often enough to keep flap-cycling the
   latch instead of releasing it.

## The fix (CeHooks.java only; two surgical changes)

- `ROW_TOTAL_CAP = 512` replaces the flat `total > 64` refusal. Genuinely
  dense cells (mobs-25k ~700/cell, items-100k ~830/cell) still bail to the
  native Rust filter; spread's 100-300-candidate rows now scan in Java at
  ~40-70ns/candidate (~10-20us total) instead of the ~13-30us fixed JNI
  toll. Exactness contract untouched: home re-check + live-box filter +
  ARENA/getter resolve are per-candidate and unchanged.
- The latch now releases the moment the running fast EMA stops beating the
  mirror EMA at the SAME 4/5 boundary `wantFast` uses
  (`FAST_EMA*4 >= MIRR_EMA*5 -> FAST_LATCH = 0`). True-win buckets keep the
  latch (spread w2b0: ~15us fast vs ~2.9ms mirror); mobs-25k w2b0 drops
  back to mirror duty.

## Local verification (2-core sandbox, spread-100k, MODULE=on, 600 ticks)

Counter deltas over the last 100 measured ticks:

| metric | CI @6a56dd9 | local @a633c61 |
|---|---:|---:|
| rowBig refusals | 10.6k/tick | **~350/tick** (199k total) |
| fast-path total cost | 208 ms/tick | **12.3 ms/tick** |
| small-box queries routed | fast (JNI) | mirror (EMA-honest on 2 cores) |

Local p50 839.7ms is NOT comparable to CI 4-core numbers (2 cores, 2.5G
heap); the routing story is the signal: the query toll the ceiling
regression added is gone, and the gate is back to EMA-honest routing
everywhere it was measured.

## Expected CI (a633c61)

- spread-100k-mod: 517.8 -> ~400-440 (back to -8..-15% vs vanilla).
- mobs-25k-mod: 285 -> ~240-250 (latch stops paying 3x on 4.6k q/tick).
- items-100k-mod: unchanged ~48.8 (rows ~830/cell stay native; latch idle).
- mixed-150k: mix of both effects; the cancelled leg runs this time.

## What remains (next levers, honestly Rust-accelerable)

1. **Sorted/windowed rows** — rows store candidates homed in a 16x16 cell;
   a push box touches ~2.6/16 of the x-range. Publishing rows sorted by
   center-x (or with conservative packed x-bounds per id) would let the row
   scan skip 85-95% of candidates without dereferencing the Entity
   (~10-20us -> ~2-4us per mid-density query). Publish-side sort cost must
   be amortized (dirty rows only) or made incremental.
2. **Native query toll** — the JNI path pays ~5-30us fixed (crossing, drain,
   publish, array copy, ARENA resolve). Batched queries (per-tick push
   sweep) would amortize it, but require restructuring vanilla's per-entity
   call sites — deferred.
3. Tracker/dispatch floors (doc 14) — unchanged, Java-structural.

---

# ADDENDUM — L4c CI verdict + L4d (partial revert + windowed rows)

## L4c on CI (run 39763715530, commit a633c61, 9/11 legs before supersede)

| scenario | vanilla | module | delta | was @6a56dd9 |
|---|---:|---:|---:|---:|
| items-100k | 95.6 | 48.0 | **-49.8%** | -49.1% (unchanged, as predicted) |
| spread-100k | 464.8 | 361.3 | **-22.3%** | +10.0% REGRESSION -> fixed |
| mobs-25k | 447.3 | 467.8 | **+4.6%** | -36.3% -> BROKEN by the latch release |

## The latch release was a self-fulfilling cold-path trap

Releasing the latch stopped the fast stream in mobs-25k w2b0. The JNI
route went cold: re-probes priced at ~348us (qMs sampled phase nanos
agree) vs ~13us while the stream was warm. Everything fell back to a
full-load mirror — 18.7k queries/tick, `AABB.intersects` share 13.5% ->
26.5% — and the leg lost its win. A stale-high fast EMA can never snap
down to the warm price without the stream that warms it, so "release
when the EMA says fast loses" always confirms itself.

The spread regression L4c chased was the row CEILING all along: spread
w2b0 hysteresis alone routed fast (15us vs 2.9ms mirror) even with the
latch armed. Lesson recorded: **a latch that gates a warm path must be
judged at its warm operating point, not against an EMA trained while
the path was idle.** The existing break (`dt > 4 x MIRR_EMA`) still
releases a latch that genuinely loses.

## L4d (commit 13d65cc)

1. Revert of the latch release (latch semantics back to 6a56dd9).
2. `ROW_TOTAL_CAP = 512` kept (the actual spread fix).
3. **Windowed rows**: each published row entry packs
   `(id, xLo, xHi)` — the entity's x-interval in 1/1024-block units
   relative to the row's cell origin (floor/ceil + 2-unit slack, 16 bits
   each, conservative superset of the true interval — property-tested).
   The Java row scan skips x-disjoint candidates at ~3ns without
   dereferencing the Entity; survivors run the unchanged exact path
   (home re-check + live-box intersects + predicate). Pending same-cell
   box changes leave a row's window stale for that id until the next
   publish — the pending scan records such ids (WIN_IDS) and bypasses
   the window test for them; cross-cell/removal pending entries keep
   the existing refusal/null-resolve paths.

Expected: spread w2b0 row-served at ~2-5us (was 15-30us JNI), mobs-25k
dense cells native under the restored latch, items-100k unchanged.

## L4e (probe-time batch draining) — the cold-trap root cause

The L4d mobs-25k re-run STILL landed in the all-mirror basin
(`fast=21,225 mirror=21,618,570`, re-probes ~368us). Root cause found in
the flush counters: `flush=21,681 / 18,739,443` — **865 probe entries per
drain**. The batch drained only when a fast query happened (18 probes/tick
x 1/1024 duty); without a fast STREAM the batch grew across ticks, and
each rare fast query paid the whole backlog — apply ~865 entries plus the
dirty-row publish of everything accumulated — at cold-JIT prices. The EMAs
honestly priced that at ~348-368us, preferred the 9us mirror, and the
fast path never streamed again. The 6a56dd9 "-36%" state was a lucky
basin: its latch was set during early warm phases and the stream kept
itself warm.

Fix: drain the batch at PROBE time, every 64 entries (`DRAIN_CAP`), and
raise the row path's pending tolerance to the same 64 (`PENDING_CAP`,
WIN_IDS resized). The total apply+publish work is invariant — only its
placement moves from queries to probes. Consequences:

- a fast query is now PURE (~2-8us): no backlog to drain;
- probe prices become honest and stable, so the latch/bootstrap can set
  on merit instead of early-phase luck;
- `bN <= 64` between drains keeps the zero-JNI row path servable
  (previously `bN > 16` refused it constantly in moving-entity worlds).

Expected: mobs-25k recovers the latched basin (or better — dense queries
on the row path), spread keeps the L4c win via row-served mid-density,
items-100k unchanged.

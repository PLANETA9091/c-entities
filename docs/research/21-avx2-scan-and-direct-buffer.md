# 21. The mixed regression strike: AVX2 scan + zero-copy candidate return

Date: 2026-10-23. Directive: kill the doc-20 open regression
(mixed-150k **+15.3%**) and push every scenario harder — "maximum
percentage boosts, zero regression". Everything below is measured or
proven; the A/B verdict table is at the end (CI, paired same-runner).

## 1. Where the 11 µs actually went

Doc 20 left two named suspects (JNI candidate return, worldId memo) and
a measured fact: mixed ran 20.3M fast queries at EMA ~11 µs. The raw
index (no JNI, no Java) was profiled with a synthetic mixed-shaped
workload (`examples/query_bench.rs`): 64 piles x 4000 items in a
+-12-block footprint (=> ~1500-2500 candidates/cell — the density the
11 µs EMA implies at ~20 cycles/candidate), 25k mob boxes, half inside
piles.

Scalar scan measured **6647 ns/query**. The index itself — not JNI —
is the mixed cost: in mixed, items move every tick ((tickCount+id)%4
cadence), so pile cells go dirty constantly, the zero-JNI row path
stays dead (rowOv / pending rows), and every query lands in the native
scan of a multi-thousand-candidate cell. The per-cell loop was scalar
f64-promoted compares: ~20 cycles per candidate, sequential.

## 2. The blocked-SoA layout (centities-core)

Cell boxes re-blocked from stride-6 into **blocks of 8 candidates with
six contiguous segments of 8 f32** (x0,y0,z0,x1,y1,z1):

```text
offset(candidate i, segment s) = (i >> 3) * 48 + (s << 3) + (i & 7)
```

Same bytes per candidate as stride-6 (144 B per 8 candidates), but each
segment is ONE 8-wide vector load — zero shuffles, zero gathers. Whole
new blocks are pre-filled with **self-rejecting padding** (x0 = +INF
fails `x0 < qx1`, x1 = -INF fails `x1 > qx0`); the last block's lanes
past `n` are masked out of the movemask, so stale data after
swap-removes is unreachable. Mutation paths (cell_add / cell_remove /
in-place upsert / cell_scan_packed / check_invariants) all speak block
math now; the invariant checker asserts whole-block capacity.

## 3. The AVX2 prefilter + exact re-check (the no-regression proof)

`simd.rs` runs the vanilla intersects test on 8 candidates per
iteration (6 loads, 6 compares, 5 ANDs, 1 movemask), then every
survivor goes through the **unchanged f64-promoted exact compare**
before its id is pushed. The prefilter bounds are the exact bounds
expanded by **EPSF = 1e-2 blocks**:

* stored coords: f32 cell-relative (|v| < 96) or absolute f32 y;
* the prefilter bound is `round_f32(qx1 - ox) + EPSF` — the f32
  rounding of a cell-relative query bound is <= ULP(64) = 7.6e-6,
  which is **1300x below EPSF**;
* therefore `x0f < round_f32(qx1 - ox) + EPSF` holds whenever the
  exact `x0f + ox < qx1` holds (same argument for all six faces);
* so the prefilter is a **provable superset** of the exact check — it
  can never drop a candidate the scalar path would keep. NaN cannot
  occur (finite coords), padded lanes reject themselves.

Runtime detection (`is_x86_feature_detected!("avx2")`), scalar
fallback compiled for every target; all `unsafe` lives in simd.rs
behind a safe dispatcher (`scan_cells`) — the crate keeps
`deny(unsafe_code)` everywhere else. Differential parity tests
(`tests/simd_parity.rs`) force both modes on the same index and assert
IDENTICAL results: dense piles + boundary boxes, an adversarial
exact-boundary grid (faces ON query bounds, sub-ULP jitter, degenerate
zero-width boxes), and a 6k-step mutation storm (block-boundary adds,
removes, moves) — plus the existing 9 exactness tests now run through
the SIMD path against brute force.

Local bench (`cargo run --release -p centities-core --example query_bench`):

| workload | scalar | AVX2 | delta |
|---|---:|---:|---:|
| dense-pile mob query (mixed shape) | 6647 ns | **1034-1316 ns** | **~5-6.4x** |
| spread mob query | 55-65 ns | 67-69 ns | ~1x (parity noise) |

## 4. Zero-copy candidate return (queryNative2)

Doc 20's second lever. A direct ByteBuffer is allocated once at arm
(`setActive`), registered native-side via `nativeSetOutBuffer`
(`GetDirectBufferAddress` + `NewGlobalRef`; the previous registration
is `DeleteGlobalRef`'d on re-register). `queryNative2` writes result
ids with **raw pointer stores** into that pinned memory — the per-query
`get_array_length` + `set_long_array_region` JNI ops and their copies
are gone. Java reads ids through the intrinsified `ByteBuffer.getLong`
(one Unsafe load each) into the reused QBUF, then the resolve loop is
untouched. Regrow contract: `-n` (n <= -2) = required capacity; the
batch was already consumed, so the retry does NOT re-apply it — same
semantics as the legacy array regrow. The legacy `queryNative` stays
registered and armed: `IDS_BUF == null` (any registration failure) or
a vanished buffer (queryNative2 returns -1) falls back to it. The
shared flush+query core is extracted (`locked_flush_query`) so both
natives cannot drift apart.

## 5. worldId 1-slot pair memo

The 13.28% self-time lead from doc 20 (JFR inline attribution, treated
with suspicion — but the fix is free). `worldId` now checks ONE
atomically-published pair (`AtomicReference<Object[]{level, wid}>`)
before the 4-slot scan: queries and merge scans come in same-level
bursts, so a single identity compare hits ~always. The pair swap can
never tear level/wid apart (two plain volatile fields could). Slot
scan + map path remain the fallback.

## 6. What was NOT done (and why)

* **target-feature=+avx2 for the whole cdylib** — rejected: auto-
  vectorization of the item-tick f64 math risks nothing semantically
  (no FMA feature enabled), but it makes the whole binary SIGILL on
  pre-AVX2 CPUs. The explicit-intrinsics + runtime-detection route
  gets the win on the hot loop with zero compatibility surface.
* **Java-floor levers (tracker/EntityTickList)** — deferred; the scan
  and the return path were the measured costs.
* The gate/EMA logic is UNTOUCHED — the parity protector keeps its
  exact feedback dynamics.

## 7. CI A/B verdict (26.3, paired same-runner, mean MSPT)

PENDING — the paired A/B (`ab-paired.yml`) is dispatched on this
commit; this table is filled from `reports/ab-paired/SUMMARY.md` when
the run lands. Expectation from the local numbers: mixed's 20.3M fast
queries drop from ~11 µs toward the 1-3 µs band (dense cells ~6x
cheaper natively), which should flip the +15.3% regression to parity
or better, and every scenario where the gate routed to mirror on price
(spread) should re-route to fast. The gate, not this doc, decides.

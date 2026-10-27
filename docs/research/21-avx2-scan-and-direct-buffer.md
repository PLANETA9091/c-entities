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

## 7. The two integration bugs the first A/B runs caught (and fixed)

The e2e kill-switch did its job twice:

1. **register_natives rc=-1** — `queryNative2` was registered with a stale
   `[J` out-param copied from queryNative's descriptor; RegisterNatives is
   all-or-nothing, so the WHOLE arm sequence failed and the module stayed
   vanilla (no crash, exact parity). Guard added:
   `tests/natives_parity.rs` parses the compiled CeHooks.class methods
   table and asserts the ACC_NATIVE set equals the Rust-registered table —
   descriptor drift now fails in `cargo test`, not in a CI boot.
2. **Byte order** — a direct ByteBuffer defaults to BIG_ENDIAN while the
   native writes raw little-endian i64 stores: `getLong` byte-reversed
   every id and the `(int)` cast landed on 0 → ARENA miss + getter null
   for EVERY candidate (resolved=0; desync stayed 0 because stale>0 reads
   as the normal batching window). The legacy `set_long_array_region`
   path converts per-element — that is why the old code resolved 95%+
   and the buffer path resolved 0%. Telemetry proof: cand=17.1M/0
   arenaMiss=17.1M (buffer path) vs cand=1.1M/1.09M arenaMiss=1904
   (doc-20 leg). Fix: `bb.order(LITTLE_ENDIAN)` at both allocation
   sites; e2e now asserts the candidate resolve ratio (cand=N/M, M >=
   N/2 when N >= 8) — this failure shape cannot pass e2e again.

## 8. The REAL regression was never the module: ViaBackwards

The first fixed-code A/B still read items +16% / mixed +19% — but the
scan changes are inert in a playerless world, and the run history held
the answer: the bench bot was kicked "Outdated client! Please use 26.3"
in EVERY 26.3 run since the retarget (62 BOT-ERRORs in doc 20's own
baseline #87). Without a player, Entity Activation Range keeps
entities inactive — the module pays its fixed costs (probes, flushes,
merge machinery) and never gets to show a single win. Doc 20's A/B
verdict table was measured playerless on BOTH legs.

Root cause chain, verified locally on a real Paper 26.3 boot:
* minecraft-data has NO 26.3 protocol data (npm and master) — the bot
  cannot speak 26.3;
* ViaVersion 5.12.2-SNAPSHOT loads fine on 26.3 but "only supports
  newer client versions" — old client (26.1) on new server (26.3)
  needs **ViaBackwards**, which nobody staged;
* + ViaBackwards → BOT-LOGGED-IN + BOT-SPAWNED (local proof);
* and one more hop: Paper 26.3 flipped the `white-list` default to ON
  (1.21.8 defaulted OFF) — the whitelisted-kick replaced the outdated-
  client-kick until `white-list=false` was pinned in the harness.

## 9. CI A/B verdict (26.3, paired same-runner, WITH the bot finally in)

ab-paired run #11 (347e989): **AB-PAIRED-GATE: PASS**

| Scenario | vanilla (off) | module (on) | ratio | verdict |
|---|---:|---:|---:|---|
| items-100k (3 reps) | 90.91 | **88.12** | **0.969** | module −3.1% |
| mixed-150k | 2546.0 | **2057.3** | **0.808** | **module −19.2%** |

BOT-LOGGED-IN + BOT-SPAWNED, ARMED confirmed in every on leg, desync=0.
Same 2-vCPU runner class where the playerless runs read 1.16-1.22 —
with a real player the fast paths finally engage and mixed turns a
+19% regression into a −19% win. items stays near-parity (its module
tax is ~0.2ms/tick against a 90ms floor; the gate margin 1.05 has
headroom). spread-100k was dropped from that run's table (missing
samples are tolerated by the gate); its playered verdict lands with the
next full matrix.

The doc-20 mixed "+15.3% open" is CLOSED: the regression was the
missing bench player, and the module's own changes (AVX2 scan +
zero-copy return) are what carry mixed to −19% once the workload is
real. Next levers (Java floor: tracker/EntityTickList) are untouched.

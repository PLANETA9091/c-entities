# doc 55 — wave15: THE F-KILL (the parallel executor's serial fragments die)

Branch `main` @ wave15. The arm: `CE_SER=1` restores the serial fused walk
(the A/B control); the default IS the parallel executor (wave14).

Session arc: wave14 shipped the double-buffer shadow eval at **1.49 ms/tick
p50 @ 25k** (2 vCPUs, −82.3% vs the vanilla 8.41) with the scaling fit
`par(T) = F + W/T`, **F = 0.44 ms** (the serial fragments), W = 2.12 ms.
Wave15 kills the fragments: **F ≈ 0.17–0.25 ms** (−2.2×), same-window A/B
**−25…−29% vs the serial arm** (wave14 measured −17%), 100k × 400:
7.0–7.3 → **5.1–5.2 ms**. The −90% line now crosses at **FOUR real cores**
(par(4) ≈ 0.76–0.83 ms = −90…−91%); eight cores ≈ 0.46–0.50 ms = −94%.

## The instrument: the phase probe

`PHASE_PROBE` (a static gate + per-phase `Instant` laps, drained by
`phase_probe_take()`); the `#[ignore]d` `phase_probe` test drives the
bench-shaped 25k world (400 stabilize, 200 timed):

    BEFORE (wave14 v2): players 0.000 | pre 0.252 | pass1 1.763 | walk 0.082 | pass2 0.060 | tail 0.000 | drawers/tick 157
    AFTER  (wave15):    players 0.000 | pre 0.100 | pass1 1.999 | prefix 0.000 | pass2 0.058 | tail 0.000 | drawers/tick 157

(the test binary inflates absolute walls ~25% vs the bench binary — the
phase RATIOS are the signal; the bench numbers below are the ledger's.)

The decomposition that drove the design: the parallel eval itself is NOT
the serial bottleneck — the fragments were the alive-map scan, the four
per-tick memsets (`repair_buf`/`assign_buf` clears, the stats staging),
and the serial stats-contract walk.

## The shape (what died, what replaced it)

Five whole buffers and two serial passes DIED:

- `alive_idx` (the serial O(n) push-scan) — replaced by the **rank
  prefix**: ONE serial pass counts each chunk's alive slots and assigns
  its `base_k` in the same breath (the count IS the prefix — no second
  loop). A parallel count pass was BUILT first and measured SLOWER: the
  rayon dispatch of a 25 KB flag scan costs more than the scan itself
  (~20 µs serial) — the stage stays serial by measurement.
- `results_buf` (the per-slot (stat, drew) staging) — pass 1 writes the
  contract stat DIRECTLY through the chunk's own **stats region**: the
  k-range `[base_k, base_k + alive_count)` is contiguous per chunk, so
  safe `split_at_mut` tiles `[0, alive_len)` into disjoint regions that
  ride the par zip. Every stats word is written EXACTLY ONCE — the
  per-tick memsets are gone (the buffer only resizes when the population
  actually grows).
- `repair_buf` + `assign_buf` + `drawer_list` (the global repair set) —
  replaced by the **chunk-local drawer ledger** in `WorkerBufs`:
  `(slot, k, drew)` in k order, appended during pass 1.
- **the serial walk** (the O(alive) stats copy + the drawer stream
  pre-assignment) — replaced by the **stream prefix**: a serial
  O(chunks) loop hands each chunk its `base_stream` = the chain advanced
  by every PRIOR chunk's draws. Bit-exact because alive order IS
  ascending slot order — a slot-chunk owns a contiguous k-range, so the
  chunk pieces chain in chunk order exactly like the old per-mob walk.
  `advance_stream` = two 48-bit LCG steps per draw (the same constant
  chain); the executor's post-tick stream = the chain end.
- pass 2 keeps the proven threshold: **serial below 1024 drawers** (the
  25k bench world draws 157 mobs/tick — 0.6% of the population, 0.058 ms),
  **chunked-parallel above** (each chunk walks ITS ledger off its
  `base_stream`, re-clones A→B, re-evals, re-travels, overwrites its own
  stats region; empty-ledger chunks exit in one branch).

The `debug_assert` pins the draw-count value-independence per drawer per
tick (the re-eval's draw count must equal the ledger's).

## The W14c verdict (kept out; the number stays)

The lazy-snapshot undo (the full-MobAI snapshot at the first structural
write — the W14c candidate that was in flight when wave14 landed) stays
dead per doc 54's measurement (10.5k snapshots/tick → +0.75 ms). Wave15's
probe adds the sharper verdict: **pass 2 costs 0.058–0.062 ms total** (157
drawers × ~0.38 µs) — the gate pre-pass that would eliminate pass 2
entirely (extract the stroll gate into a pre-eval replay) risks
single-source-of-truth drift in `ai_tick` for ~40 µs of wall. Not worth
the invariant.

## Numbers (same-window A/B, same host, 2 vCPUs, bench binary)

- 25k × 400, windows in sequence:
  - W1: PAR 1.32/1.33/1.25 vs SER 1.81/1.79/1.77 → −27%
  - W2 (box drift +0.2 ms — even the untouched serial arm moved):
    PAR 1.51/1.52/1.51 vs SER 2.02/2.00/1.98 → −24.5%
  - W3: PAR 1.41/1.41 vs SER 1.87/1.99 → −25…−29%
  - CE_JOBS sweep (W2 window): 1.74 / **1.40** / 1.35 / 1.37 / 1.40 for
    jobs = 1/2/4/8/16 — the default 2 stays; the dial is for real hosts.
- 100k × 400: PAR 5.12/5.23 vs SER 7.01/7.31 → −27…−28%.
- vs the vanilla 8.41: the 2-core box lands at −83…−84% (windows drift);
  the FIT is the deliverable:
  - T1 = 2.52–2.77, T2 = 1.34–1.52 across windows → **F ≈ 0.17–0.25**,
    W ≈ 2.35–2.52.
  - par(4) = F + W/4 ≈ **0.76–0.83 ms = −90…−91%** ← the −90% goal at
    FOUR real cores
  - par(8) ≈ 0.46–0.50 = **−94%**, par(16) ≈ 0.32–0.33 = **−96%**

## Proof

- Census: the twin gate `parallel_executor_matches_serial_executor` —
  400 ticks × (chase + stroll-far) shapes, the whole-store FNV dumps +
  the stats stream + the handoff words + the post-tick stream
  bit-identical against the serial arm, on the NEW shape.
- Suites: **105 Rust tests green** (37 core lib + the core integration
  suites incl. exactness 10 + 20 native incl. the twin + 22 sim + aux;
  the `#[ignore]d` phase_probe rides along as the instrument). The module
  suite stays CI-side (mapped 26.3 jar + Java 25).
- Zero per-tick allocation on the steady state: the stats regions are
  re-split per tick from the persistent buffer (the split Vec is
  pointer-sized per chunk), the ledgers keep their capacity, the worker
  bufs persist.

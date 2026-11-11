# doc 54 — wave14: THE PARALLEL EXECUTOR (the per-mob independence theorem made hardware)

Branch `main` @ wave14. The knob: `CE_PAR=1` (`NativeTick::set_parallel`).
Session arc: the serial fused walk (wave12b/13: **1.80 ms/tick p50 at 25k**)
→ the parallel arm: **1.49 ms (5/5 rounds, −17% vs serial, −82.3% vs the
vanilla 8.41)**; 100k × 400: 7.06 → **5.63**; the raw scaling 1 → 2 threads:
2.56 → 1.50 (**1.71×**).

## The theorem

The tick decomposes across mobs because:

1. **No mob reads another mob.** The folds answer every scan over the packed
   players; `move_tick` touches self fields only (the fused-vs-split
   executor twin, wave9). The snapshot is immutable during the tick.
2. **The ONE cross-mob channel — the shared stroll stream (`target_rng`) —
   is a PURE DRAW COUNTER.** Whether a mob draws is a function of its own
   state + its own seed LCG (the stroll gate runs BEFORE the draw and never
   reads the stream); at most one draw per `ai_tick`; the advance is TWO
   48-bit LCG steps per draw regardless of the values (the park arm draws
   2×16 via `lcg_next`, the fold arm 2×16 via `stroll_draw_lcg16` — the
   same LCG).

So the serial mob order fixes WHICH mobs draw and HOW MANY; the VALUES chain
as advance-per-draw. Every mob's result is a pure function of (its pre-state,
the immutable snapshot, game_time, its assigned stream value) — the parallel
result is bit-identical to the serial walk for ANY thread interleaving.

## The arm (v2, the shipped shape)

- **pass 1 (rayon chunks):** each worker CLONES its slots A→B
  (`clone_from` — the pre-tick state stays pristine in A), evals B IN PLACE
  with a placeholder stream value, and FUSES the travel right after (the
  wave9 vanilla order, per mob). Non-drawers are already CORRECT.
- **walk (serial, one fused pass):** the stats contract copy + the drawer
  pre-assignment — each drawer's value = the chain advanced by the draws
  before it; the executor's post-tick stream = the chain end.
- **pass 2 (the drawer repair):** re-clone ONLY the drawers A→B, re-eval
  with the assigned value, re-travel. Serial below 1024 drawers; chunked
  parallel above. `WorldRes::draw_count` counts the stream touches; a
  debug_assert pins the draw-count value-independence.
- **swap:** the shadow buffer becomes the store; no copy pass. Zero
  per-tick allocation (pre-sliced result regions + persistent per-worker
  eval workspaces `WorkerBufs`).

Twin gate: `parallel_executor_matches_serial_executor` — 400 ticks ×
(chase + stroll-far) shapes, whole-store dumps + the stats stream, the
handoff words AND the executor's post-tick stream bit-identical. The
executor twins + the knob probes serialize on `EXEC_TWIN_LOCK` (the suite
runs tests concurrently; a knob flip landing between the two arm ticks was
a harness race, not an executor bug).

## The v3 measured dead-ends (kept out; the numbers stay)

The clone exists to keep a pristine re-eval source. Three cheaper-looking
shapes were built and measured on the 2-core dev box — all lost to the
clone:

- **The full-MobAI snapshot at the first structural write** (the undo log
  with `UndoInfo.snap`): the bench counted **10.5k snapshots/tick** (the
  lookaround starts + the melee applies + the strolls all count as
  structural writes) — 10.5k × (alloc + 252 B copy + drop) ≈ +0.75 ms —
  the arm fell to 2.25 ms p50.
- **The per-write journal** (`UndoJournal`, 16 B per WRITE site execution,
  ~25 hooks across `apply_actions`/`start_vanilla`/canuse/cc, the bits
  recomputed post-restore): correct (the twin drove it green once the
  entry capture also carried the TRAVEL-written fields — the re-eval reads
  the position the fused travel moved!) — but the per-mob entry capture
  (14 fields) rides INSIDE the eval's critical path and its scattered
  read-modify-write per mob cannot hide behind the 2-thread memory
  pipeline: **1.87-2.02 ms p50**.
- **The explicit DRAM prefetch** (1 then 4 cache lines, 8 slots ahead, both
  arms): **no measurable effect** — the eval's stall is not the slot's
  first-line latency.

The clone wins because it is DENSE: the A→B copy pre-reads the eval's
input (a software prefetch pipeline), the eval then runs L2-hot, and both
stages parallelize. At 2 cores the clone's extra traffic is cheaper than
the journal's critical-path captures.

## The scaling law and the 8-core projection

`par(T) = F + W/T` with the measured (25k, CE_PAR): par(1) = 2.56,
par(2) = 1.50 → **F = 0.44 ms** (the serial fragments: the alive map, the
walk, the repair dispatch, the players build), **W = 2.12 ms** (the
clone + eval + travel). Projection: 4 threads ≈ 0.97, **8 threads ≈ 0.71
ms/tick = −91.6% vs vanilla**, 16 ≈ 0.57. The −90% line sits between 4
and 8 real cores; the box in hand (2 vCPUs) tops out at 1.49.

## Proof

- Census: the twin's whole-store dumps match bit-for-bit every tick of
  400 × 2 shapes; the stats stream (`Vec<AiStat>`, the contract order)
  matches; the executor's post-tick `target_rng` matches.
- Suites: **126 Rust tests green** (37 core, 10 exactness, 8 itemcore,
  21 native incl. the new twin, 22 sim, the rest aux). The module suite
  stays CI-side (mapped 26.3 jar + Java 25).

## Numbers (interleaved A/B, same host, 2 vCPUs)

- 25k × 400, 5 rounds alternating: serial 1.78 / 1.81 / 1.80 / 1.82 / 1.80
  vs parallel **1.47 / 1.51 / 1.50 / 1.51 / 1.49** — **−17%, 5/5**.
- 100k × 400: serial 7.05 / 7.08 vs parallel **5.68 / 5.59** (the extra
  population compounds the parallel win: the traffic halves across the
  threads while the compute divides).
- vs the vanilla 8.41: **1.49 = −82.3%** at 2 cores; the 8-core projection
  crosses the −90% line.

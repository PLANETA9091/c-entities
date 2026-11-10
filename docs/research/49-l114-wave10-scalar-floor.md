# Doc 49 — L114 WAVE10: THE SCALAR FLOOR (measured, pinned, and the last drop taken)

## Verdict first

25k bench (seed 12648430, interleaved A/B, same binary, 5×400-tick runs):
**p50 2.48 vs 2.60 ms/tick (−5% median, 5/5 wins)** — the wave10 odd-tick
skip vs the full-floor arm (`CE_NO_ODD_SKIP=1`). 100k: **9.59 ms/tick**,
chasing **14008 == 14008** (the exact wave8 world). 25k chasing
**3745 == 3745** in every A/B run. **122 suites green**, including the new
`odd_skip_matches_full_floor_executor` twin (400-tick whole-store dumps,
skip arm vs full-floor arm) and the 33 s natives_parity differential (the
JNI-shaped wire path untouched).

Session cumulative from the L112 start: **8.41 → ~2.4 = −71%** (the box
noise band today is ±5%; doc 48's window measured 2.36 on the same seed).

## The wave's real product: the floor is MEASURED, not assumed

Three probes landed this wave (all env-gated, one relaxed load off-path):

1. **The scaling curve** (`native_bench` 5k→25k): 96 → 105 → 108 → 109 →
   108 ns/mob. The per-mob cost SATURATES from ~15k — the 25k working set
   (~14 MB) lives in L3. **The MobAI shrink (440 B of static goal table →
   88 B of live half) will NOT pay at 25k**; it is a 100k-only lever
   (57 MB vs L3 there).
2. **The AI/travel split** (`CE_NO_TRAVEL=1`): full 2.65 vs AI-only 2.12
   ms/tick → **AI ≈ 85 ns/mob (≈255 cycles), travel ≈ 21 ns/mob**.
3. **The floor's phase split** (`CE_FINE`, Instant-bracketed, corrected
   for the bracket padding): **update (the canUse evals) ≈ 79%** of the
   floor, **cleanup ≈ 17%, lock rebuild ≈ 4%**. The evals are the
   workload: ~7-8 gate evaluations per even tick per mob, each an
   LCG draw (5-6 cycle serial chain) + loads + compares — the measured
   44 ns/mob sits ON the op estimate. The scalar walk is at its
   latency floor.

Dead ends measured and closed this wave:

- **`-C target-cpu=native`**: 0 effect (2.52-2.63 vs 2.51-2.55). The
  workload is dependency/branch-bound, not ISA-bound.
- **The const-table specialization (ZombieSrc)**: the zombie goal table
  (kinds/prios/flags/cfg — 66 words) baked into a compile-time `Src`
  behind the `pack_fp == ZOMBIE_FP` gate (fnv1a over exactly those
  words). Twin-gated, world-exact — and **±0 on the bench** (the
  200-tick interleaved runs said +0.5-1% ZS, the 400-tick runs said
  −1.5%: noise-dominated, the icache cost of the unrolled walk eats
  the const-fold gain). **REVERTED** — `zombie_table()`/`static_fp`
  stay `const fn` (infrastructure), the third monomorphization does
  not ship.

## The ship: the odd-tick no-op skip (the maintained every-tick bit)

The 26.3 odd branch is `tickRunningGoals(false)` — it touches ONLY the
`requiresUpdateEveryTick()` goals (LOOKAROUND + MELEE in the classified
set): zero RNG draws, zero state writes of any other record, and
`done()` writes the LCG value back verbatim. When NEITHER every-tick
goal runs, the whole odd floor is a provable no-op — the exact shortcut
the Java fast path has run since L22c (`aiMeleeRunning`). The Rust side
ran the full walk (Ctx build + 2×11-record scan) to discover nothing.

- `MobAI::et_running` — the bit, maintained at EVERY start/stop:
  `apply_actions` (ACT_GOAL_START / ACT_RLA_START / ACT_GOAL_STOP /
  ACT_GOAL_STOP_D), the swarm resolver's start bodies, AND the
  production resolver's (`WorldRes::start_vanilla` — the folded-vs-
  parked twin caught exactly this site missing the note on run one;
  the gate works).
- `ai_tick`: the skip fires before the fold build — tick_count/no_action
  still advance, the seed/states stay untouched, `AiStat::default()`
  returns.
- `CE_NO_ODD_SKIP=1` restores the full floor (the A/B arm + the twin's
  B side). The `et_bit_and_fp_pins` unit test pins the maintenance
  helpers and the fp table-sensitivity.

## What did NOT move (honest negatives)

- **The JNI production wire**: still the per-round protocol. wave10's
  original plan (n_mob_ai_tick) remains the production half of the
  merge — but it cannot move the bench number (the bench never crosses
  JNI), and the protocol rework is a full session on its own.
- **The SIMD batch walk** — the only remaining order-of-magnitude lever
  (see Next).

## Next (the honest map)

1. **The SIMD batch walk** (the 3-5× lever): the evals are independent
   ACROSS mobs and serial WITHIN one (the LCG chain). An 8-lane AVX2
   batch over the even-tick eval stream: SoA live state (seed/running/
   states/nav/pos), the 64×64 LCG mul emulated in 3×vpmuludq per step
   (8 lanes at once), the gate compares branchless, the divergent tails
   (a gate passes) fall out to the scalar walk for that mob. Parity
   surface: the per-lane draw sequence must be bit-exact — the
   batch-vs-scalar executor twin over whole-store dumps is the gate,
   the same shape as this wave's twins. Estimated: a full session per
   record family (target first — 3 of the 8 evals, then stroll/lookat/
   lookaround), the cleanup/lock halves ride along after.
2. **n_mob_ai_tick** (the production JNI protocol — doc 48's promise):
   one call per mob-tick, the round loop in Rust, the resident slot as
   the wire. Bench-invisible; production-visible.
3. **The MobAI live-half shrink** — only worth it at 100k+ populations
   (the scaling curve says the 25k box is L3-resident).

## Tooling notes

- The sandbox rootfs cycled full again (the stale bench worlds from the
  ab8/parity-mobs runs — 1.9 G — were the consumers; deleted; the
  `PAPER_JAR` must now be passed ABSOLUTE: the build.rs walks up from
  the jar path for `libraries/` and a relative path breaks the walk
  from cargo's build-script CWD).
- The work/jdk25 copy is broken (missing `lib/server/libjvm.so`) — use
  `/home/z/tools/jdk-25.0.5+8`.
- The bench knobs this wave: `CE_NO_TRAVEL` (the AI-only row),
  `CE_FINE` (the floor phase split), `CE_NO_ODD_SKIP` (the A/B arm).

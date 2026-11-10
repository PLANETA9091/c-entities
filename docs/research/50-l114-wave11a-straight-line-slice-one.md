# Doc 50 — L114 WAVE11a: THE STRAIGHT-LINE WALK, SLICE ONE (the engine overhead is the cost)

## Verdict first

25k bench (seed 12648430, interleaved A/B, same binary, 5×400-tick runs):
**p50 2.34 vs 2.53 ms/tick (−7.5% median, 5/5 wins)** — the straight-line
zombie walk (`ai_floor_z0`) vs the generic engine (`CE_NO_Z0=1`). 100k:
**9.79 ms/tick**, chasing **14008 == 14008**; 25k chasing **3745 == 3745**
in every run. 108 suites green — including the new
`zstrait_matches_generic_executor` twin (**400-tick whole-store dumps,
PASS first run**) and the 34 s natives_parity differential. The
centities-module suite did not run this session (the sandbox recycled;
the JDK is gone) — the JNI wire is untouched by construction: z0 is a
resident-only path behind the wave9b merge.

Session cumulative from the L112 start: **8.41 → ~2.34 = −72%**.

## The wave's discovery: the engine overhead IS the cost

A standalone probe (`eval_probe.rs`, kept) measured the bare idle-path
eval stream — the 11 canUse bodies + the 6-draw LCG chain, nothing else:

- **Mode A (per-mob scalar): 22.8 ns/mob.**
- Mode B (8-mob software-pipelined, the LCG latencies interleaved):
  37.2 ns/mob — **the interleave LOSES**: the AoS gather (~10 fields ×
  8 mobs into lane arrays) costs more than the hidden mul latency. The
  SIMD/interleave batch is dead on this layout; do not revisit without
  a SoA store.
- The engine's whole AI measured ~85 ns/mob (wave10). **~60 ns/mob
  (≈70%) is Ctx builds, running rebuilds, the phase machine, per-record
  plumbing, out-buffer bookkeeping — NOT the eval math.**

That inverted the plan: the batch walk was the wrong big lever; the
engine-overhead kill is the right one, and it is SCALAR.

## The ship: ai_floor_z0 — slice one (the idle even-tick shape)

One straight-line function in goals.rs (~500 lines), no Ctx, no phase
machine, no running arrays:

- **The gate** (every bit checked before any draw): the zombie table fp
  (`pack_fp == ZOMBIE_FP`), the fold contract on, the EVEN branch, NO
  running goals — the cleanup and lock rebuild are then provably empty
  and the lock tables start cleared.
- **The body**: the 11 eval bodies verbatim through the same `Src` view
  (live table reads — no const-table assumption): the HURT_BY gate, the
  three NEAREST interval draws + the scan/LOS-fold/find-target-fold
  tails, SPEAR, the two ZA throttles + the canuse fold, MTV, the stroll
  fold (the external stream + ACT_STROLL_SET), the lookat scan + LOS
  fold, the RLA draw-exact start body (next_f64 cos/sin + nextInt(20))
  on the floor's own LCG.
- **The locks**: a 5-word local per selector; `can_replace_all` and
  `lock_only` replicated exactly — INCLUDING the in-stream lock gate
  (after rec1's LOS-fold start, recs 2/3 skip on the FL_TARGET lock —
  caught in self-review, the twins would have caught it in CI).
- **The parks**: MTV's tail, SPEAR's non-empty mainhand, the non-fold
  tails write the EXACT park words (cursor/seed/n_act) — the 32-round
  loop resumes through the generic engine from round 1 (cursor != 0),
  which rebuilds from the applied state. The displaced-stop machinery
  is unreachable in-slice (the table's per-flag priorities ascend with
  the record order) and the gate keeps it that way.
- **The exits**: ST_DONE + the seed words + the stroll ext tail — the
  same wire contract `ai_tick` already applies.

Everything outside the shape — odd ticks with running every-tick goals
(the wave10 skip already eats the empty ones), chasing mobs, strollers,
non-zombie tables, the parked flow — rides the generic engine
bit-for-bit.

## Honest accounting

The −7.5% is the IDLE-shape slice only. The coverage math: the
ineligible mob-ticks (running goals: chasing 15%, strolling, RLA) still
pay the full generic walk, and the z0 body keeps the action pushes, the
apply, the seed handoff and the fold build. The remaining ~50 ns/mob of
engine overhead lives in the shapes z0 does not take yet:

1. **Slice two — running ≠ 0**: the cleanup (the cc fold, the stops),
   the lock rebuild from the running set, the update stream under
   locks. The displaced-stop machinery becomes reachable — the full
   lock_only semantics. This is where the chasers (the most expensive
   mobs) live.
2. **Slice three — the odd-tick running body** (RLA/melee tick) and the
   resume rounds.
3. The apply/seed/fold-build tail is shared by every shape — worth its
   own pass once the slices land.

## Dead ends (measured, closed)

- The 8-mob software-pipelined eval stream (above) — the AoS gather
  eats the latency win.
- (wave10, carried) target-cpu=native: 0; the const-table-only
  specialization: ±0 — the overhead was never the static words, it is
  the machinery around them.

## Environment notes

- The sandbox recycled mid-session: the local checkout reverted to an
  old commit (origin/main held the wave10 tip — recovered via
  `git reset --hard origin/main`), rustup + the JDK + paperboot263 are
  GONE. Rebuilt rustup (1.99.1). The JDK + the paper boot need a
  re-download before the next module-suite run (recipes in doc 47/48).
- The bench knobs this wave: `CE_NO_Z0` (the A/B arm), plus the
  wave10 set (`CE_NO_TRAVEL`, `CE_FINE`, `CE_NO_ODD_SKIP`).

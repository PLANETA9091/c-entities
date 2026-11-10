# doc 52 — wave12: the third pass dies + the ar-bit hole

Branch `main` @ wave12a+b. Session arc: **8.41 → ~1.90 ms/tick p50 (−77%)**
(the wave11c addendum's 1.95 minus wave12's honest yields, same-host
interleaved).

## wave12a — the melee approach refresh FUSES into the main loop

The tick opener ran the approach refresh as a PRE-PASS: a full second
population stream per tick (25k × ~360 B of L3/DRAM traffic) for a
per-mob two-field write (`approach_x/z` ← the target's snapshot position,
or `target_player = -1` on the stale index). The write is independent of
every other mob — no mob-mob reads exist (the folds answer every scan
over the packed players; the snapshot is immutable during the tick) — so
landing it immediately before the mob's OWN ai_tick (inside the fused
loop, where the mob's cache lines are already L1/L2-hot from the write
the AI is about to pay for) is bit-identical.

- `prepass_matches_fused_refresh` — the executor twin: both arms,
  whole-store dump per tick, 400 ticks, PASS first run.
- `CE_PREPASS=1` / `set_prepass(true)` restores the pre-pass shape (the
  A/B arm).
- Executor row (CE_NO_AI): **0.43 → 0.38 ms/tick** (−12% of the fixed
  executor cost).
- Interleaved A/B 25k: **2.20 vs 2.02 p50 (−8.2% arm-to-arm, 3/3
  rounds)**.

## wave12a FIX — the ar_running hole in the PRODUCTION resolver

The first full-suite run of the wave failed the folded-vs-parked twin:
`the maintained ar_running bit drifted from the live records`
(left: false, right: true) — deterministic on clean main (wave11c
`996c9b6`), invisible to the release bench (the assert is debug-only).

Root cause: `world.rs::start_vanilla` — the PRODUCTION resolver's start
bodies — maintained `et_note_start` (the wave10 bit) but never
`ar_note_start` (the wave11b bit). The sim resolver's copy
(swarm-sim mob.rs) had it; the action-mirror sites (apply_actions
ACT_GOAL_START / ACT_RLA_START / ACT_GOAL_STOP) had it. Every
fold-resolved start drifted `ar_running` false while the record ran, and
the window mob-ticks (start → first stop) routed by the wrong gate.
The maintenance audit is now closed: all six production `running = true`
writers + the stop path carry the bit.

The bench census was byte-identical before and after the fix (the bench's
start/stop pattern never hit the window on an even tick) — the fix is a
correctness gate, not a bench lever. But it is the twin system doing its
job: a real production-resolver defect caught by the debug_assert at the
z0/z1 gate on the first suite run.

## wave12b — the standing fixed point (the stationary travel shortcut)

`move_tick`'s early-out: when no movement branch fired (`zza == 0.0`)
and the delta is exactly zero (`dx == dz == 0.0`), the whole travel
chain degenerates to one store (`dy = OG_GRAV_TAIL`):

- the zero-input returns `(0,0,0)` before the LUT (lengthSqr < 1e-7) —
  no RNG;
- `move_sets_position` is FALSE for dy = OG_GRAV_TAIL (dy² ≈ 6.1e-3 ≫
  1e-7) — no setPos; on a fresh mob's first tick (dy = 0.0) the guarded
  writes are `x += 0.0` — bitwise idempotent for x ≠ −0.0, and −0.0 is
  unreachable (the only writers are `x += mx` with mx ≠ −0.0 and RN
  addition of +0.0);
- the drag tail writes `+0.0` over `+0.0` — dx/dz can never be −0.0
  (their only writers are products with positive constants).

All the dump twins hash dx/dy/dz/x/z bits per tick — 19 native twins
green. Measured yield: **at the host noise floor (≤2%)** — the travel
slice moved 0.51 → 0.49 ms in the slice probe, within run-to-run drift.
Kept: provably identity-preserving, pinned by twins, free. The
tiny-nonzero-dx mobs (the multiplicative decay never reaches exact 0.0)
take the full chain — their dx decay is observable state.

## Dead ends measured, closed

- **GoalRec 20 B → 16 B** (state i32→u16): cfg/cfg2 carry f32 BITS (the
  LOOKAT probability 0.02 and the context range — the parity values
  themselves), so they cannot narrow below 32 bits; the remaining
  ~4 B/record is ~44 B/mob of DRAM — a ~1-2% lever at 100k only, not
  worth the churn. The wave11c 40→20 B took the real win.
- **face() without the two divisions** (`atan2(ddx, ddz)` directly, the
  scale-invariance): mathematically equal, but the shipped form's f64
  rounding is the parity contract; the f32 narrowing makes a mismatch
  ~2e-10 per call — not PROVABLY never. The bit-exactness bar forbids.
  The atan2 itself (glibc f64, ~15 ns/mover) is the travel floor.

## Numbers

- 25k: **1.90–1.98 p50** on today's noisy host (quiet-host runs 1.82);
  interleaved A/B arm-to-arm −8.2% (wave12a).
- 100k × 400: **7.55** chasing **14008 == 14008** (ledger 7.82).
- 25k chasing **3745 == 3745**; census byte-identical
  (even_z0 21.5% / even_z1 28.6% / odd_skip 32.0% / odd_z2 17.9%,
  even_gen 0 / odd_gen 0).
- Suites: **111 Rust tests green, 0 failed** (19 native twins incl. the
  new prepass gate; the module suite stays CI-side — needs the mapped
  26.3 jar + Java 25).

## The honest ceiling (the standing statement)

The remaining budget decomposes (release slice probe, stabilized
population): executor skeleton ~0.27, AI ~1.4 (the RNG-serial eval
stream — the draws ARE the semantics, bit-exactness locks the stream),
travel ~0.49 (the movers' glibc atan2 ~15 ns is bit-locked; the
streaming is one fused pass). Within the standing constraints —
bit-identical parity, no threading, hard math only — the architecture's
measured floor is ~1.6-1.7 ms at 25k on this host. The next honest
magnitude would need to relax a constraint, not another wave of math.

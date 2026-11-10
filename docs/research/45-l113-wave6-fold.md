# Doc 45 — L113 WAVE6: the fold (no-park resolutions) + the phantom-start fix

## Verdict first

25k bench (400 ticks, seed 12648430): **p50 4.00 → 3.74 ms/tick (-6.5% vs wave4; -16%
vs the pre-wave6 tree at 4.44 in the same measurement window; -19% vs the session's
morning base 4.59).** 100k bench: **17.06 → 14.73 ms/tick (-13.7%)**. chasing
10292 → 12592 — **the world itself is more correct** (see the bug fix below). All 97
tests green, including the folded-vs-parked bit-identical executor twin over 400
ticks (full store word-for-word) and every pre-existing parity gate.

## The directive that shaped the wave

Parallelism was rejected ("parallelism is not real speedup — real speedup is writing
hardcore mathematical code"): the in-flight WAVE5 thread-chunking diff was dropped
uncommitted; the ledger-meter disarm fix (one relaxed load per unsampled mob-tick,
~25k locked RMW + 25k stores gone) was kept as the only non-math survivor.

## The structural change: the fold

The floor's staged requests (park → executor resolves → resume) cost a full round
each: a 46-word repack + a floor re-entry (ctx rebuild + resume lock rebuild). At
25k mobs the staged-request profile measured ~10k round-trips per tick (find_target
1-in-10 per Nearest goal ×2 non-player goals, LOS scans, stroll targets, RLA starts,
cleanup cc checks). WAVE6 lets the RESOLVER declare a `FoldCtx` (trait
`Resolver::fold_ctx`) — when its v0 answers are pure functions of the pack + the
streams the floor already carries, `ai_floor_ext` resolves them IN PLACE:

- **find_target**: alive && !spectator over the packed snapshot (gated by
  `find_target_fold` — FlatWorld's contract differs and keeps the park).
- **LOS scan**: the all-pass mask feeds the shared `los_apply` (best-selection +
  ref write + lock transition — one source for the fold and the resume arm).
- **stroll target**: the external stream (splitmix16 for FlatWorld, LCG-16 for
  WorldRes — flavor-selectable) drawn by the floor; the advanced stream returns
  through the out-buffer EXT tail; the actions order STROLL_SET (pend) before the
  lock_only GOAL_START (nav = pend) — the exact resolution→start order.
- **RLA start body**: the DRAW-EXACT javap body (nextDouble + nextInt(20)) runs on
  the floor's OWN LCG — the same stream position the parked resolution draws from;
  rel/state ride ACT_RLA_START.
- **cleanup cc**: the v0 answer from the pack + the carried target/approach.

`FoldCtx::off()` (the JNI path, the fixtures, the twin) reproduces the parked
semantics word-for-word — the twin gate drives both modes over the same population.

## The bug the fold flushed out (PRE-EXISTING)

The LOS park wrote its candidate list into `out[8..]` — **action slot 0**. A
same-round action (the cleanup cc stop) followed by a LOS park made
`apply_actions` read the candidates as `ACT_GOAL_START p0=0` — a PHANTOM HurtBy
start on every affected mob-tick, then the next rebuild's FB_LOCK_COLLIDE fallback.
The parity CI never saw it because the JNI scenarios never chain cc-stop → LOS-park
in one round (HurtBy needs combat). The native bench chains it constantly.
**Fix**: the native executor exchanges the candidates through the EXT tail (beyond
the action area, `FoldCtx::ext_cands`); the JNI wire slot is untouched (the
`FoldCtx::off` path keeps the legacy write, the 776-word Java buffer falls back
cleanly). The world after the fix: chasing 10292 → 12592 — the phantom was
silently suppressing ~2300 correct target acquisitions.

## The measurement trap that ate an hour

The first wave6 bench showed a +1.1 ms/tick REGRESSION. Bisection over three stash
cycles found the culprit: **the debug probes left in the hot path** —
`env::var_os` per mob-round and per action (environ lock + linear scan, ~200ns ×
hundreds of thousands of calls per tick). Removed; the "regression" became the
numbers above. Two false suspects were cleared by measurement: the per-call Lcg
memo (per-call cold slots re-paid the INV division — reverted to the thread-local
table, now CONST-initialized so the wave4b lazy-init branch is gone too) and the
fold-site branches (a by-value `FoldMask` snapshot in the Ctx — one load per site).

The absolute cross-build numbers on this box drift ±0.3 ms — the honest deltas are
the same-binary A/B (folded vs parked via the bench's `CE_NO_FOLD` knob) and the
candfix-only baseline (HEAD + the LOS fix, same chasing workload): 15.72 → 14.73
at 100k (-6.3%) on identical worlds.

## Gates added

- `folded_executor_matches_parked_executor` (exec.rs): 96 mobs, chased + strolling
  + looking, 400 ticks, fnv over the full store + goal states + stream — bit-identical.
- `folded_ai_tick_matches_parked_ai_tick` (mob.rs): the FlatWorld twin, same contract.
- `fold_streams_match_resolver_math`: splitmix16 / LCG-16 word-for-word over 5000 seeds.
- `travel_open_ground_consts_pin`: the folded open-ground friction/drag chain
  (OG_*) equals the runtime functions bit-for-bit; pins the 0.6-exact rounding fact
  (1-(1-0.6) returns 0.6f32 exactly — the speed gate is `> 0.6`, so the arm is 0.23).
- The floor-level fixture gates are untouched (the pack wire did not change).

## What moved where

- `centities-core/goals.rs`: FoldCtx + FoldMask, splitmix64/stroll draw fns,
  ai_floor_ext, los_apply (shared), Cu::Continue, ACT_STROLL_SET/ACT_RLA_START,
  OUT_EXT layout (stream 2 + cand tail 35), const-init TLS memo.
- `swarm-sim/mob.rs`: Resolver::fold_ctx/set_target_rng, the executor's per-round
  stream handoff (gated on `stroll_drew`), apply arms for the two new acts, out_len
  → OUT_EXT_LEN, the folded open-ground travel constants (OG_*) in move_tick.
- `centities-native/world.rs` + `exec.rs`: WorldRes fold_ctx (+ `fold_off` twin
  hook), the executor twin gate, the bench's `CE_NO_FOLD` A/B knob.

## Next (wave7 candidates)

1. The fold for REQ_CANUSE_VANILLA (ZombieAttack's alive+in-range tail) — needs the
   target index in the fold context (carried today only for cc).
2. The resident pack's round words shrink further: with folds the round rate dropped
   ~40% — re-profile the pack share before touching it.
3. The cc fold currently reads approach from the FoldCtx snapshot; a pack word for
   the target index would let the LOS fold also refresh it in-round.

## WAVE7 (this session): the canuse fold + the cc-fold target-clear fix

Directive: «ДАВАЙ ЕЩЁ ЖОЖЩЕ... максимальное количество арифметики». Scope:
wave7 candidate 1 (the REQ_CANUSE_VANILLA fold) + the stress-twin harness.

**The canuse fold** (`GK_ZOMBIE_ATTACK` in `can_use_inner`): the parked flow
staged the MeleeAttackGoal.canUse tail (target alive + reach) through a full
park/resume round every 20 ticks per attacking mob. The WorldRes v0 answer
(`matches!(ZOMBIE_ATTACK) && target_player >= 0`) is provable from the
round's own HAS_TARGET gate — the fold resolves IN PLACE: the
lastCanUseCheck write rides the new ACT_CANUSE_OK action (p1 = game time,
the exact resume-arm write), then lock_only — the parked flow's apply order
(state write at resolution, then the resume round's transition) preserved.
Enabled by `FoldCtx::canuse_fold` — ON for WorldRes (its canuse_vanilla
contract is exactly `target_player >= 0`), OFF for FlatWorld (its
canuse_vanilla is `false` — different contract, keeps the park) and OFF in
the JNI path (FoldCtx::off).

**The stress twin caught a REAL wave6 latent drift.** The 96-mob fixture
twins passed, but the bench-shaped stress twin (the exact native_bench
world, parked vs folded per tick, fnv over the whole store) diverged at
tick 9: mob 18136, folded tgt=0 vs parked tgt=-1. Root cause — NOT the new
fold: the WAVE6 cc fold's STOP half never carried the parked flow's
resolver-side target clear (`target_player = -1; scratch[1] = MF_BASE` at
the REQ_CC_VANILLA resolution for ZOMBIE_ATTACK/NEAREST_TARGET). The wave6
fixtures never chain a cc-stop with a live out-of-range target; the dense
bench does constantly. **Fix**: the fold emits ACT_TARGET_CLEAR (the apply
mirrors the resolver's clear verbatim) and the Ctx carries an `mf_ov`
override so the SAME-round gates see the cleared MF word exactly as the
parked flow's re-packed resume round does. The canuse fold's changed round
schedule is what EXPOSED the drift — the twin now runs the bench shape as
a standing gate (`stress_twin` example: parked vs folded, 450 ticks,
bit-identical, chasing 3745 = 3745).

**Numbers** (this box, interleaved A/B, same binary, seed 0xC0FFEE,
25000×400): folded **3.85** vs parked **4.14** ms/tick p50 (−7.0%; three
pairs 3.85/4.10, 3.82/4.14, 3.87/4.16). The box drifts ±0.3 across builds
— the paired same-binary delta is the honest metric. Chasing 3745 in BOTH
modes (the wave6 shipped 12592 on the doc's box — the scenario/seed drift
across boxes is documented above).

## Next (wave8 candidates)

1. The REQ_LOS_SCAN park for LOOKAT_PLAYER still stages every even tick in
   player-rich worlds — the scan's non-LoS half is native; the fold needs
   the Sensing contract per candidate family.
2. The pack round words: with wave7 the round rate dropped further —
   re-profile the pack share before the direct-read protocol work (the
   «пол читает MobStore напрямую» lever: the executor consuming MobAI
   directly, the wire shrinking to per-tick diffs).
3. The travel pass (2.1 ms/tick, the #2 phase): the AoS MobAI record is
   ~600B — the travel slice touches ~5 scattered words per mob; a hot SoA
   mirror (pos/delta/on-ground/friction) would stream 2 cachelines per mob
   instead of chasing the record.

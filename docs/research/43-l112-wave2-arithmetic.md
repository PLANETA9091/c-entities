# L112 — wave2 arithmetic: zero-alloc executor, incremental grid, honest clocks

Dedirective: «продолжи ещё жожще оптимизировать с аритметиками и вообщем всем».
Scope: the native executor hot path (`centities-native` + `swarm-sim::mob`),
measured by `native_bench` (25k zombies + 1 player, 400 ticks). Parity gates:
`chasing` count bit-identical (3036), `reused_out_buffer_matches_fresh_zero`
(bit-for-bit twin run), `cell_of_matches_div_euclid_bit_exact` (dense sweep),
full workspace tests green.

## Baseline → wave2 (same box, same seed)

| metric    | before            | after             |
|-----------|-------------------|-------------------|
| p50       | 8.06–8.15 ms/tick | **4.9–5.1 ms/tick** (−38%) |
| p90       | 8.4               | 5.1–5.3           |
| p99       | 11.4              | 10.5–10.7         |
| wall 400t | ~3.3 s            | ~2.15 s           |
| chasing   | 3036              | **3036 (bit-identical)** |

## The strikes

1. **Honest clocks (profile.rs PhaseGuard).** The guard paid a full
   `Instant::now()` (~20–25 ns) PER SCOPE PER MOB even when the phase was
   not sampled — 4 scopes × 25k mobs ≈ **~2 ms/tick of pure clock reads
   inside the measured wall time**. `t0: Option<Instant>` — the unsampled
   path is one relaxed load. The ledger's own numbers were inflated by
   this (ai 5.59 was partly the meter's shadow).
2. **Zero-alloc pack (mob.rs `pack_into`).** `pack()` allocated + memset
   a fresh `Vec<i32>` (~536 B) PER ROUND PER MOB — 25k mobs × ~2.4 rounds
   ≈ 60–75k allocations per tick. Now: one `pack_scratch: Vec<i32>` per
   EXECUTOR, grown on demand, fully overwritten each round (`pack()` kept
   as the exact-len fixture entry; single impl, no divergence).
3. **Shared out buffer (mob.rs OUT_RESET=9).** The floor's write
   discipline is verified: done/park write out[0..6], fallback writes
   {0,5,7}, `Out::push` writes full act slots — every executor read is
   write-before-read EXCEPT the fallback RNG handoff pair (out[3],out[4]).
   So one out buffer per executor + a 9-word reset per mob replaces the
   3 KiB per-mob memset. Gated bit-for-bit by
   `reused_out_buffer_matches_fresh_zero` (dirty buffer pre-filled
   0x7BAD_C0DE, twin mobs, 400 ticks, full state compared each tick).
4. **Devirtualized resolver (mob.rs `ai_tick<R: Resolver + ?Sized>`).**
   The `&mut dyn Resolver` vtable call in the request arm is gone;
   concrete resolvers monomorphize (`WorldRes<OpenGround>` inlines
   `nearest_player_in_range`).
5. **Incremental SeenGrid (store.rs — the Horizon N1 trick, their
   seen_grid update(i,b)).** The per-tick rebuild cleared the map and
   re-hashed every mob (25k hash inserts/tick; grid phase 1.81 ms). The
   superset contract only needs every alive mob listed under its CURRENT
   cell, so: walk slots once, re-bucket ONLY movers (stored cell ≠
   computed cell), detach-on-remove, invalidate-on-compact. Cells persist.
   Grid phase → **0.64–0.75 ms** (−2.4×). Mover edge cases gated by
   `incremental_rebuild_tracks_movers_and_recycling`.
6. **Strength reduction in cell_of.** `x.div_euclid(4.0)` →
   `x * 0.25` (+floor): division by a power of two is exact in IEEE-754 —
   bit-identical cell for every finite x (dense sweep test pins it), and
   the rebuild walk stops burning 2 fdiv per mob per tick.
7. **face() reuses the caller's d2.** move_tick computed
   `ddx*ddz` squared distance, face() recomputed it before sqrt — same
   product bits, so face() now takes d2 and sqrts it (2 flops saved per
   moving mob, bit-identical yaw).
8. **Resolver hoisted** out of the per-mob loop in exec.rs (one
   WorldRes per tick, target stream advanced through it).

## The profiler grew sub-phases (swarm_sim::ledger)

The phase ledger said "ai is hot"; the new SUB ledger splits ai into
pack / floor / reslv / acts, clocked 1-in-17 MOBS inside the 1-in-17
sampled tick (1-in-289 mob-ticks) and reported as a percentage split —
because the first version clocked EVERY mob of the sampled tick and its
~240k Instant::now calls inflated p99 from 10.5 to 22 ms (the meter's
shadow again — now bounded: p99 10.5–10.7).

## Wave3 strike map (from the split)

floor 42% | pack 28.9% | acts 20.4% | reslv 8.7%.

- **pack delta-writes**: rounds 2+ only rewrite the round-variable words
  (cursor/res/req_op/xyz/seed/mf/nav/goal-words) — apply_actions updates
  the packed words in the same write site it mutates live state; the
  invariant "scratch == live state" holds by construction. ~70% of pack
  words skipped per round.
- **floor internals**: the walk is a pure function (in→out), so an
  arithmetic rewrite (branchless lock-bit scans, single-load flag words,
  fewer per-round Ctx re-init) is parity-gated by the fixtures + the
  reuse gate.
- The top-level ledger still carries sampled-tick clock inflation (ai est
  5.71 vs real total p50 5.12) — the honest per-phase ms/tick needs the
  perf lane (blocked: no root in this sandbox; CI native-profile keeps
  the perf attempt per commit).

## Wave3 (same session): delta-pack — the scratch maintained, not rebuilt

The wave2 split said pack 28.9% — but the real cost hides in the round-0
FULL pack every mob pays (idle mobs: 1 round = full pack; they are the
majority). Wave3 makes the scratch a MAINTAINED mirror instead of a
rebuilt snapshot:

- `apply_actions(out, players, packed: Some(scratch))` mirrors EVERY
  mutation into the packed word(s) it owns (running, goal states, the
  LOOKAT state word via the shared `packed_lookat_state`, no_action,
  nav_state, mf-has-target).
- The request arm mirrors the mutations that live OUTSIDE apply:
  `repack_goal_words(gi)` after the resolver's start_vanilla and the
  melee canUse lastCanUseCheck write; `scratch[1] = MF_BASE` when the
  cc-chain drops the target.
- Rounds 2+ (`cursor != 0`) write ONLY the round-variable words:
  cursor/mf/no_action/nav_state/seed/res-op/xyz — ~11 words instead of
  134 (12× fewer).

**The invariant probe** (debug builds only): every delta round packs a
fresh FULL probe and asserts word-for-word equality — the FIRST run of
the probe caught two real drifts (the melee state write and the
resolver-start mutations), fixed, and now every debug test is a delta
gate. Release keeps only the ~11-word delta.

Result (8-run medians, same box/seed): p50 5.06 → **4.78 ms/tick**
(total now **−41%** vs the 8.06 baseline; chasing still 3036,
bit-identical). Split after wave3: pack 28.5% | floor 42.4% |
reslv 8.3% | acts 20.8% — the pack share barely moved because the
round-0 full pack (the idle majority) is untouched; wave4 candidates:
(1) per-mob RESIDENT pack (write the 96 static words once at add_mob,
~50 mutable words per tick), (2) the floor's internal walk arithmetic
(pure function — parity-gated rewrite).

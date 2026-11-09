# L112 — wave4: the resident pack + the armed-atomic gate + the 5-flag lock table

Directive: «ещё жоще давай» (on top of wave2 «продолжи ещё жожще оптимизировать
с аритметиками и вообщем всем»). Scope: the same native hot path
(`swarm-sim::mob` + `centities-core::goals` + `centities-native::exec`),
measured by `native_bench` (25k zombies + 1 player, 400 ticks, canonical seed
0xC0FFEE). Parity gates: `chasing` bit-identical (3036), the debug invariant
probe extended to EVERY round, three NEW twin gates (resident-vs-full at mob
and executor level, shared-scratch mob switching), full workspace green.

## Baseline → wave4 (same box, same seed)

| metric    | wave3 (baseline)  | wave4             |
|-----------|-------------------|-------------------|
| p50       | 4.94 ms/tick      | **4.00–4.06 ms/tick** (−19%) |
| p90       | 5.14              | 4.22–4.29         |
| p99       | 10.67             | 9.55–9.58         |
| wall 400t | ~2.12 s           | ~1.74 s           |
| chasing   | 3036              | **3036 (bit-identical)** |

Cumulative native pivot: 8.79 → 4.00 (−54%); from the wave2 8.06
start point: **−50.4%**. Reference (doc 41): module ~242–260,
vanilla ~494 — the native executor now runs the same population ~60×/122×
faster than the Java paths it replaces.

## The strikes

1. **THE RESIDENT PACK (mob.rs PackState).** The aiPack wire buffer splits
   into three word classes:
   - STATIC per mob TYPE — goal cfg words {0,1,2,3,6,7} × 11, selector
     counts, follow range, the static-zero lastHurtByMobTimestamp — written
     ONCE per fnv1a fingerprint change (`pack_state_static`; `MobAI::pack_fp`
     computed at construction — the goal table is fixed at add_mob, the same
     contract wave3's delta-pack already relied on);
   - TICK-static — game_time, player count, the player snapshot block —
     written ONCE per (game_time, pcount, player-fnv) stamp change
     (`pack_tick_words`); `tfp` is computed ONCE PER TICK by the caller
     (`MobAI::tick_fp`, hoisted out of the mob loop — the per-mob fnv was
     itself ~0.1 ms/tick before the hoist);
   - ROUND words (~46: cursor/mf/tick_count/id/no_action/nav/seed/res/op/
     res-xyz/pos/eye/goal running+state) — written EVERY round by
     `pack_into_resident` — down from pack_into's 134.
   25k mobs × 88 saved words/tick ≈ 2.2M redundant word-writes/tick gone;
   the player block alone was 12 words × 25k = 300k writes/tick for one
   player. The wave3 delta arm (rounds 2+) is SUBSUMED — the round words
   are cheap enough to write every round, which also removes the mirror
   subtlety for the words pack_into_resident re-derives live.

2. **THE PROBE CAUGHT A REAL DRIFT (the reason the discipline exists).**
   The first resident shape keyed the round-1 arm on `cursor == 0` and wrote
   the request slots (IN_RES=14, IN_RES_OP=15, IN_RES_X=16..22) as ZEROS.
   The debug probe (full pack_into word-compare, now run EVERY round) failed
   at word 15, round 0: a park at (SEL_TARGET=0, PH_CLEANUP=0, gi=0) produces
   the resume token 0 — the RESUMED round flows through the same arm as the
   fresh round and its request words MUST ride the pack. The zombie table's
   FIRST goal (HurtBy, target selector, cleanup phase, idx 0) parks exactly
   there in every chase — release builds would have silently corrupted the
   chase chain (res=0 ⇒ "predicate failed"). Fix: the request words are
   round words, written from the live (res, req_op, rx, ry, rz) EVERY round.
   Wave3's ledger lesson repeats: the invariant probe is the parity gate
   that pays for itself on the first ride.

3. **THE ARMED-ATOMIC GATE (goals.rs can_use).** The ML_EVAL_SWAPS
   `fetch_or` stamp ran UNCONDITIONALLY per can_use eval — ~275k locked
   RMWs/tick on ONE cacheline (25k mobs × 11 gates) even with the mob
   ledger disarmed. Now gated behind `ML_ON.load(Relaxed)`: one relaxed
   load (~1 ns) instead of a locked RMW (~10–20 ns) per eval; the OR fires
   only inside a dump window. This was invisible to the sub-ledger (it
   clocks the floor phase from OUTSIDE) — the first cost the Rust-side
   reasoning found without perf.

4. **scan_players_into — the malloc-free scan (goals.rs).** The player
   scan returned `Vec<usize>` — a heap allocation per scan, TWO scans per
   mob per even tick (target combat + lookat) = ~20M mallocs per bench.
   Now a caller-provided `[usize; 32]` stack buffer (the scan runs only
   under the MF_MANY_PLAYERS guard ⇒ P ≤ 30 ⇒ 32 is a hard cap).

5. **MAX_LOCK_FLAGS = 5 (goals.rs).** The lock table is per FLAG BIT and
   vanilla Goal.Flag has FIVE ordinals — the classified set uses bits 0..3.
   The per-round rebuild wrote 32 dead words per floor call (16 bits × 2
   selectors); now 10. Bits ≥ 5 in a RUNNING goal's mask collides-fallback
   (FB_LOCK_COLLIDE) — the vanilla mirror takes over, the same outcome a
   real lock collision produces; bits 0..4 behave exactly as before.

6. **apply_actions de-Optioned (mob.rs).** `packed: Option<&mut [i32]>` +
   8 × `as_deref_mut()` per action → a plain `&mut [i32]` (the executor
   always has the scratch) + an `n == 0` early exit. The executor owns the
   scratch by construction; the Option dance was a branch per mirrored
   action for a case that cannot happen.

7. **exec.rs plumbing.** `PackState` + the reused `players_buf` (the old
   code allocated a fresh `Vec<PlayerSnap>` every tick) + `set_full_pack()`
   (the wave3 semantics for the twin gates). tfp computed once per tick.

## The gates (wave4 additions)

- `resident_pack_matches_full_pack_twin` (mob.rs): twin mobs, one full-pack
  PackState, one resident — 400 ticks with a chased player, bit-compare
  x/z/seed/yaw/nav/target/lookat/goal running+state EVERY tick.
- `resident_pack_survives_mob_switching` (mob.rs): two mobs alternating on
  ONE shared scratch (the production shape) vs per-scratch controls — the
  resident path must rewrite every per-mob word on owner switch.
- `resident_executor_matches_full_pack_executor` (exec.rs): two 64-mob
  NativeTick executors, `set_full_pack(true)` vs production resident —
  400 ticks, the whole store (id order, x/y/z/seed/yaw/nav/target/dx/dz/
  all 11 goals' running+state) bit-compared every tick.
- The debug probe now covers EVERY round (fresh AND resumed) — it caught
  the token-0 drift on its first run.
- All wave2/wave3 gates unchanged and green (`reused_out_buffer_matches_
  fresh_zero`, the pack fixture tests, the parity fixtures, determinism).

## Ledger honesty note

After wave4 the sub-ledger (pack/floor/reslv/acts, 1-in-289 mob-ticks) is
at its resolution floor: the sampled mob's measured phase time includes
2 × Instant::now (~50–100 ns) which now rivals the REAL phase cost
(pack work dropped below the meter's shadow). The split (pack 26.0% /
floor 42.2% / reslv 9.2% / acts 22.6%) is half meter-overhead from here
on — the next attribution must come from `perf` (native-profile CI on a
linux-tools runner) or a finer design. The wall-clock p50 remains the
honest verdict metric.

## Next (wave5 candidates, in expected-value order)

1. **SoA hot columns for the MobStore** — the tick streams the ~590 B AoS
   MobAI 3–4 times per tick (grid / approach refresh / ai loop / travel)
   ≈ 50–60 MB of bandwidth per tick at 25k mobs; the travel phase (2.05 ms)
   is mostly that streaming. Hot columns (x/z/dx/dz/seed/nav/target/lookat/
   yaw) as separate Vecs keep the vanilla insertion order and the tick
   contract; goals stay AoS (the AI working set).
2. **nextInt(bound) division** — nextInt(10)×3 + nextInt(60) per mob per
   even tick ≈ 50k runtime divisions/tick (~0.2–0.3 ms); exact u128
   ceil-magic needs an across-calls cache to beat the div — thread-local
   2-slot (bound, INV) cache is the candidate; the rejection-loop
   semantics must stay bit-exact.
3. **perf on the CI runner** — the linux-tools path of native-profile.yml
   gives the first true per-FUNCTION Rust table at these magnitudes.

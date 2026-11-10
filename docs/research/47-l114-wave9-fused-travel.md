# Doc 47 — L114 WAVE9: the travel pass fuses into the AI walk

## Verdict first

25k bench (seed 12648430, interleaved A/B, same binary, same seed):
**p50 3.04 vs 3.38 ms/tick (−10.4%, 5/5 runs)** — the fused walk vs the
pre-wave9 split walk. The split arm stays in the binary behind
`CE_SPLIT_TRAVEL=1` and is proven bit-identical by the new 400-tick
executor twin (`fused_executor_matches_split_executor`, exec.rs): whole-
store dumps (positions/seeds/nav/goals/targets/approach/pend + the stroll
stream + population) match word-for-word every tick, plus 21 suites green
including the natives_parity differential (now runnable again — see the
tooling section).

Session cumulative from the L112 start (8.41): **−64%**.

## The lever: the second DRAM stream

The population is ~14.2 MB (25k × ~568 B: MobAI 536 B with the 11-record
inline goal array + the MobSlot registry plumbing). The pre-wave9 tick
walked it TWICE — all AI first, then all travel — and the second pass
re-fetched every mob after the AI pass had evicted it. The profiler's
honest numbers (the ledger, see the tooling fix below): ai ≈ 2.0 ms
(pack 39% / floor 42% / acts 19%), travel ≈ 1.3 ms — the travel phase was
compute-light (the fused wave6 constants, the table sin/cos) but
DRAM-heavy.

The fusion restores the VANILLA body order at the same time: vanilla
`serverAiStep` runs each mob's AI then its movement, one mob at a time —
the sim's two-pass split was a profiling convenience that predated the
folds. The fusion is legal because **no cross-mob reads exist**: the
wave6 folds answer every target/LOS scan over the packed players,
`move_tick` touches self fields only, and the stroll stream advances in
the same insertion order in both shapes. The twin proves the claim every
CI run instead of trusting the argument.

## The wave9a change (exec.rs)

- the AI loop calls `slot.mob.move_tick(&self.mth)` right after
  `ai_tick` under the same `travel_guard` phase bracket;
- `split_travel: bool` + `set_split_travel` — the twin hook keeps the
  pre-wave9 two-pass walk for the A/B and the parity twin;
- `native_bench` gains `CE_SPLIT_TRAVEL=1` (the twin arm), next to
  `CE_NO_FOLD` / `CE_FULL_PACK`.

## Tooling fixes (the ledger was blind)

- **WAVE8c regression**: `let _ = grid_guard;` MOVED the fn item without
  calling it — the whole phase ledger measured zero ticks since wave8c
  ("0 of 0 ticks measured"). Now `drop(grid_guard())`: the tick opener
  arms the 1-in-17 sample and drops immediately (a live bracket would
  book the entire tick into the grid cell — caught and fixed in the same
  pass; the first honest read after the fix produced this doc's ai split).
- **The module suite is buildable again** (the sandbox recycle ate the
  boot tree): `PAPER_JAR=<repo>/../paperboot263/versions/26.3/paper-26.3.jar`
  (the mojang-mapped 26.3 server jar — `paper-merged/paper-1.21.8.jar` is
  the OLD 1.21.8 layout and fails `net.minecraft.util.debug` /
  `world.level.gamerules`), javac from `tools/jdk-25.0.5+8`. The build.rs
  classpath walk finds `paperboot263/libraries/` (104 jars, fastutil
  8.5.18) two levels up from the versions dir.

## What did NOT move (honest negatives)

- **pack 39% of ai**: the pack is already the wave8 straight-line residue
  (head-once + 8 round words + the mirror writes); killing it means the
  wire-layout merge — the goals.rs engine reading the resident MobAI
  fields instead of the scratch words. That is a 2.6k-line bit-exact
  engine rewrite, not an afternoon patch; it stays the end-state design
  paired with the JNI protocol change (doc 46's wave9 candidate #1).
- **acts 19% of ai**: the action walk + the packed-word mirrors — the
  same merge absorbs them (the mirrors exist only to keep the scratch
  honest).
- **travel compute**: already the wave6 folded constants + the fused
  index; the phase's remaining cost was the stream, which is what wave9a
  removed.

## Next (wave9 continuation)

1. **The JNI wire protocol** (unchanged, doc 46 #1): Java packs ONCE per
   mob-tick into a resident slot; `n_mob_ai_tick` runs the 32-round loop
   in Rust; CeHooks.aiRun's per-round packIn + 2 JNI array copies die.
2. **The wire-layout merge** (this doc's pack/acts negative): after the
   protocol change the scratch IS the resident slot — the engine reads
   it directly, the pack and the mirrors disappear.
3. **The floor specialization** (doc 46 #2): the zombie table baked into
   straight-line per-goal walk code, behind a pack-level fuzz twin.

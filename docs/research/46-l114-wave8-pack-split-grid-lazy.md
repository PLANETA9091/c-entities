# Doc 46 — L114 WAVE8: the pack split + the O(1)-lazy grid

## Verdict first

25k bench (seed 12648430, taskset-pinned, interleaved A/B, same seed):
**p50 4.16 → 3.22 ms/tick (−22.6%, 5/5 runs)** — wave8c vs wave8b, and the wave8
stack against the wave7a morning window: **4.04-4.20 → 3.22 (−20%+)**. 100k:
**17.01 → 12.65 ms/tick (−25.6%)**. World parity: chasing 3745 == 3745 (400
ticks) and 14008 == 14008 (100k), every wave. 21 suites green including the
400-tick full-vs-resident executor twin (wave8a) and the folded-vs-parked
twin (which caught the wave8a seed gap at tick 0 before the bench ever saw
it). Session cumulative from the L112 start (8.41): **−62%**; from the
wave6 ledger's morning base (4.59): **−30%**.

## wave8a — the pack split (the resident wire, bench-side)

The old resident pack rewrote ~44 words EVERY round (loop over 11 goal
records + kind match + the f64 splits). But the invariant "scratch == what
a fresh pack_into would write" was already MAINTAINED, not just claimed:
every mid-tick mutation of a resident word is mirrored into the scratch by
`apply_actions` (ACT_GOAL_START/STOP/CANUSE_OK/LOOK_TICK_*/REF_SET_PLAYER/
TARGET_CLEAR/RLA_START all write both the live field and the packed word),
by the handoffs (seed, stroll stream), and by `repack_goal_words`. The
per-round rewrite was a safety net masking mirror gaps, not a source of
truth. wave8a deletes the net:

- `pack_head` runs ONCE per mob-tick (mob head + the 22 goal live words +
  the derived LOOKAT word).
- the round loop writes 8 words straight-line: cursor/res/op/rpos.
- the seed handoff lands `out[3..4]` in the scratch directly (round 1's
  seed rides pack_head; rounds 2+ ride the handoff).

**The gap the twin caught**: `start_vanilla`'s LOOKAROUND body consumes
`mob.seed` (the DRAW-EXACT start) — the parked flow's next round seeded its
LCG from the PACKED words, which the old pack refreshed from `mob.seed`
every round. With the rewrite gone the packed seed went stale → the
folded-vs-parked executor twin diverged at tick 0. Fix: `repack_goal_words`
mirrors the seed pair (the request arm's single repack site). The lesson is
the wave3 discipline restated: **one write site per field, and every
mutation site mirrors — the probe (debug, word-for-word vs a fresh pack
EVERY round) + the two 400-tick twins are the gates.**

`CE_FULL_PACK=1` = the same-binary pre-wave8 arm. Pinned 800-tick
interleaved A/B: **4.01 vs 4.20 (−4.5%, 7/7)**; 100k: **18.02 vs 20.55
(−12.3%, 3/3)** — the pack savings scale with population.

## wave8b — the grid goes O(1)

`SeenGrid::rebuild`'s detach did an O(len) `position()` scan per mover
(the 25k pile's hot cells held ~35 slots) and the cells map hashed
`(i64, i64)` with the default SipHash-1-3 (~25ns × 2 map ops per mover).
Fixes: `CellHasher` (FxHash multiply-xor — the keys are internal, a
fixed-seed cheap hash is safe; Horizon's own grid uses FxHashMap) +
`MobSlot.grid_pos` (the slot's index inside its bucket — swap_remove + the
swapped element's re-stamp). Guarded fast path with the honest linear
fallback. Worktree-interleaved A/B: **4.14 vs 4.29 (−3.5%, 7/7)**.

## wave8c — the grid goes LAZY (the big one)

The native v0 tick NEVER queries `candidates()` — the wave6 folds answer
LOS/target scans natively over the packed players, so the per-tick
re-bucket walk was maintenance for queries that never come. wave8c makes
the grid lazy: `dirty` flag fires on add/remove/compact; the first actual
query (`candidates`/`cell_len`, now `&mut` + the mobs slice) runs the full
rebuild from live positions, then answers. The grid's content is only
observable through its queries, so the superset contract is unchanged
bit-for-bit; query-heavy production worlds pay the same incremental cost
as wave8b's design (the incremental `rebuild` stays public and is what the
query path runs).

25k interleaved A/B: **3.22 vs 4.16 (−22.6%, 5/5)** — the grid phase reads
0.00 in the ledger. 100k: **12.65** (was 17.01 in the wave8a window).

## What did NOT move (honest negatives)

- **sin/cos index fusion** (travel): the fused index changes bits for
  negative yaw products — vanilla truncates `(value*K + 16384.0)` AFTER the
  float add; fusing truncates first. The save is one fmul per moving mob;
  not worth any divergence risk.
- **atan2 replacement** in `face()`: the yaw is consumed as f32 (and then
  re-multiplied by 0.017453292 for the table index); a polynomial accurate
  enough to survive f32 rounding boundaries needs ~1e-15 relative error —
  libm-grade. Bit-parity locks it; the twins compare `yaw.to_bits()`.
- **per-mob res slots** (the full Horizon residency): the slot traffic
  (~13MB/tick at 25k) fights the L1-hot shared scratch; the pack split
  got the instructions without the DRAM. The wire-layout merge stays the
  end-state design — with the JNI protocol change, not before.

## Next (wave9 candidates)

1. **The JNI wire protocol** (the promised lever): Java packs ONCE per
   mob-tick into a resident slot; the 32-round loop runs in Rust
   (`n_mob_ai_tick`), resolutions ride the out actions. The bench/native
   path IS the model (NativeTick::tick). CeHooks.aiRun's per-round
   packIn + 2 JNI array copies per round die.
2. **The floor specialization**: the zombie table is a compile-time
   constant — bake kind/prio/flags/cfg into straight-line per-goal walk
   code. Needs a pack-level fuzz twin (generic floor vs specialized floor
   over random state packs) on top of the executor twins.
3. **The floor's residual RNG chain**: NEAREST's three randomInterval
   draws per even tick are serial by bit-exactness; the only lever is
   drawing them earlier in the same chain (no parallelism allowed).

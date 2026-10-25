# 30. L22 verdict — what the first full-traffic bench measured

Date: 2026-10-25. Status: MEASURED (baseline c2140cf + e774d00, the first
runs with the L22 stack under real fill/soak traffic).

## The split

| config | mobs-25k p50 | delta vs 152.9 (L20b) |
|---|---:|---|
| block-fx gate ON, engine not armed (c2140cf) | **129.5ms** | **-15.3%** |
| block-fx gate OFF, engine ARMED (e774d00) | 193.2ms | +26.4% |
| both (projected) | ~110-120ms | — |

1. **The block-fx gate is the win**: -23ms on mobs-25k (-73% → -77.4% vs
   the 572.8ms vanilla leg). The inside-block bucket (10.6% JFR share) plus
   the fire/data tail the vanilla body rewrites per tick.
2. **The staged goal engine is a measured net loss** on the bench: the
   packer + one JNI round PER STAGED PREDICATE, and the bench's TARGETED
   zombies (the bot inside the crowd) stage a `canContinueToUse` + a
   `canUse` every even tick — 3-6 rounds/mob/even-tick at ~300ns per round
   out-costs the virtual calls they replace. The L15 Java microkernel
   (goalSelectorTick 1.05% self-time) is the cheaper scheduler at this
   request density. The engine is opt-in (`-Dcentities.mobAi=1`) until the
   pre-resolved-predicate redesign: the packer must call the vanilla
   predicates directly and pass the BOOLEANS in, with the reachability
   (lock-state) simulation moved Java-side — the engine keeps only the RNG
   and the flag math.
3. **The items-100k SIGSEGV**: the G1 SATB filter crashed on a corrupt
   pre-value under the block-fx gate's item path — `ItemEntity.tick` runs
   `applyEffectsFromBlocksForLastMovements()` + `applyEffectsFromBlocks()`
   BACK TO BACK (the mobs never exercise the ForLastMovements leg). The
   gate is mob-scoped by default (`CE_BLOCK_FX=1` re-enables the item path
   for the bisect); hs_err_pid4417 is saved.

## Shipped state (e64e66f)

- The block-fx gate: ON for Mobs by default, item path opt-in.
- The staged goal engine: opt-in; the L15 GoalSelector microkernel remains
  the production AI layer (it now arms at weave time — the zombie-liveness
  gate died with the mob floor taking over the classified traffic).
- The JNI natives + the Mob weave stay registered (inert when the flags
  are off).

## L23 — the remaining gap to -90% (mobs 57.3ms, spread 46.5ms, items 8ms)

Ranked by the measured buckets:

1. **The block-fx item path fix** (unblocks ~2-4ms on items + the spread
   legs' 60k items) — root-cause the ForLastMovements interaction.
2. **The travel/collide port for MOVING mobs** (doc 28's design; the
   travel-collide bucket is 7.5% + the intersects 5.2%) — the L6 collide
   port exists in Rust; the mob version needs the step-up branch + the
   attribute/friction reads.
3. **The tracker floor for items** (the items gap: 15.2 → 8.0ms; the
   tracker bucket is ~40% of the item profile).
4. **The goal-engine redesign** (pre-resolved predicates) if the AI mass
   matters after 1-3.

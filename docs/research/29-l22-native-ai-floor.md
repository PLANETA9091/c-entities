# 29. L22 — the native AI floor (goal engine) and the block-effects gate

Date: 2026-10-25. Status: IMPLEMENTED (Rust engine + Java floor + weaves),
pending CI. Previous: doc 28 scoped the quiet-mob movement floor; the
26.3 `mobs-25k-mod` profile at L20b (152.9ms, -73%) was FLAT — the top
frame (`AABB.intersects`, 4.5%) is the push resolve itself. This doc's
phase attribution (JFR caller-tree buckets, mobs-25k-mod @ 847934a)
located the real mass:

| share | phase | notes |
|---:|---|---|
| 28.8% | OTHER (dispatch glue, aiStep body, mid-tick tasks, JNI) | spread |
| 10.6% | inside-block-effects (applyEffectsFromBlocks + checkInsideBlocks + applier) | L22b |
| 7.8% | GoalSelector.tick (scheduler + goal objects) | L22a |
| 7.5% | travel-collide (vanilla, strolling mobs) | L23 |
| 6.6% | tracker | L14 residue |
| 6.1% | enchant-equipment iteration | L23 |
| 5.2% | AABB.intersects (push/collide candidates) | L23 |
| 5.1% | baseTick | L23 |
| 3.3% | targeting (findTarget scans) | L22a |

## 1. L22a — the native goal engine (Rust)

`centities-core/src/goals.rs` — a pure decision core over a packed i32
buffer. The Java floor (`CeHooks.mobAiFloor`, woven EarlyReturnGuard on
`Mob.serverAiStep`) packs the mob + both selectors per tick; ONE native
call (`nativeMobAiFloor([I[I)I`) reproduces the vanilla GoalSelector
sequences EXACTLY: insertion-order iteration, the flag-mask math, the
priority replacement rule (`other.priority <= locked.priority`), the
cleanup → lock-rebuild → update → tickRunning(force) phasing, and the
parity split (`(tickCount + id) % 2 != 0 && tickCount > 1` →
tickRunningGoals(false) only, target selector first).

Key semantics (decompile-verified on 26.3-223):

* **The RNG is `Entity.SHARED_RANDOM`** — ONE ThreadUnsafeRandom (a plain
  48-bit LCG `value` field) shared by every entity. The engine advances it
  natively, bit-exact with the JDK (unit-tested against `Random(42)`
  vectors: nextInt/nextFloat/nextDouble), and hands the seed back; every
  staged Java call continues the SAME stream via the seed handoff. A quiet
  zombie consumes exactly: 4 × nextInt(10) (nearest-target gates) +
  nextInt(60) (stroll) + nextFloat (lookat 0.02) + nextFloat (lookaround
  0.02) per even tick — verified by the `idle_even_tick_pulls_exactly_six_values`
  test.
* **Staging**: every decision the engine cannot prove (combat-goal
  canUse/canContinueToUse, MoveThroughVillage's side-effecting updateVisited,
  the stroll's getPosition block-scan, the lookat getNearestPlayer query,
  the findTarget scans) becomes a REQUEST; the Java floor resolves it with
  the REAL virtual call (MethodHandles for protected members) and resumes
  the engine at the parked cursor. The continuations resolve PREDICATES
  only — the engine owns every start/stop transition.
* **Transitions are actions applied IMMEDIATELY after every round**, in the
  exact vanilla order: displaced `stop()` before `start()` (the walk-back
  bug: a deferred start after a displaced navigation.stop() would cancel
  the fresh path — the immediate-apply model keeps the order exact).
* **Round cap 8** (the Java driver): the idle path needs ≤ 5 rounds
  (4 staged canUse + DONE).
* **The classified set** (exact `Zombie` class only): SpearUseGoal,
  ZombieAttackGoal, ZombieAttackTurtleEggGoal, MoveThroughVillageGoal,
  WaterAvoidingRandomStrollGoal (checkNoActionTime on, forceTrigger off,
  interval 120 → reducedTickDelay 60), LookAtPlayerGoal (Player, 8.0,
  probability 0.02), RandomLookAroundGoal, HurtByTargetGoal,
  NearestAttackableTargetGoal (randomInterval 10). ANY unknown class,
  oversized selector, unknown flag bit or triggered stroll → the mob is
  refused permanently (weak identity map) and stays vanilla.
* **Ported goal bodies** (native): RandomLookAroundGoal canUse/canContinue/
  tick (lookTime + relX/relZ reads + the lookControl writes),
  LookAtPlayerGoal canUse gate/canContinue (the alive/range bits packed at
  pack time)/tick, the stroll canContinue (`!nav.isDone() &&
  !hasControllingPassenger`), the stroll canUse pre-gates (passenger,
  noActionTime ≥ 100 — short-circuits BEFORE the RNG pull, exactly like
  vanilla), the nearest-target randomInterval gates, the spear
  mainhand-empty short-circuit.
* **The control tail** (vanilla order after the selectors):
  navigation.tick idles natively (tick++, no recompute, no path),
  moveControl WAIT → setZza(0), lookControl idle branch natively
  (setXRot(0), `rotateTowards(yHeadRot, yBodyRot, 10F)` — the exact
  Control.rotateTowards/Mth.wrapDegrees f32 ports; the cooldown>0 branch
  routes vanilla because getYRotD/getXRotD use Mth.atan2),
  clampHeadRotationToBody (`rotateIfNecessary`, maxHeadYRot virtual),
  jumpControl (setJumping(jump); jump=false).
* The byte-exact `mobAiSelectorsMirror` (the L15-floored GoalSelector
  bodies + vanilla nav/controls) finishes any ST_FALLBACK tick — the
  noActionTime++/sensing.clear that already ran are NOT repeated.

Fallback surfaces: unknown goal class (permanent refusal), engine
ST_FALLBACK (per-tick mirror), any floor exception (global disarm +
mirror-if-mutated, vanilla re-run if nothing stepped).

## 2. L22b — the inside-block-effect no-op gate

`CeHooks.mobBlockEffectsNoOp`, woven EarlyReturnGuard on
`Entity.applyEffectsFromBlocks()`. Vanilla walks every block position the
swept hull intersects (getEntityInsideCollisionShape + fluid tests +
collector stepping per non-air position) — a PROVEN value no-op when:

1. every position in the swept hull (from-box ∪ to-box — a strict
   superset of the vanilla visit set) is AIR (the vanilla lambda's own
   isAir short-circuit makes air positions do nothing);
2. the below block's `stepOn` is the empty base implementation — probed
   ONCE per Block class via `getMethod("stepOn").getDeclaringClass() ==
   Block.class` (the exact override test — Cactus/Magma/Farmland/... route
   vanilla);
3. not in rain, not on fire, not freezing;
4. `remainingFireTicks == -getFireImmuneTicks()` (the tail's rewrite is
   same-value);
5. `movementThisTick` empty (no recorded movements — portals/teleports
   route vanilla) and no debug-subscriber watching ENTITY_BLOCK_INTERSECTIONS.

The gate replicates `finalMovementsThisTick.clear()` — with an empty final
list a later `applyEffectsFromBlocksForLastMovements()` iterates nothing,
observationally identical to iterating the default movement (the gate
PROVED the hull is all-air). Hull > 64 positions → vanilla (the gate must
stay cheaper than the thing it replaces).

## 3. Numbers

Expected (mobs-25k): the goal engine kills the scheduler + idle-goal
object mass (~7-10ms/tick) + the targeting gate pulls (~1ms) + sensing
bookkeeping; the block-effects gate kills ~12-14ms/tick. items-100k and
spread-100k inherit the block-effects gate (ItemEntity.applyEffectsFromBlocks
runs per item per tick). CI validates; doc 30 will carry the measured
verdicts.

## 4. Not in this milestone (L23+)

- The vanilla travel/collide port for MOVING mobs (doc 28's design — the
  L20 gate only covers the zero-input fixed point).
- The followThePath port (strolling mobs' navigation.tick routes vanilla).
- The combat-goal body port (ZombieAttackGoal.tick stays virtual).
- The tracker floor for items (the items-100k gap).

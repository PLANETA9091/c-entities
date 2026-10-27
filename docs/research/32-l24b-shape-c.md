# 32. L24b — the moving-mob open-ground travel port (SHAPE C) + the TrackedEntity postmortem

Date: 2026-10-26. Status: SHIPPED (b2b7407 + the 99cb4fa crash fix).

## The ported slice

`LivingEntity.travelInAir` for the dominant moving shape: a grounded,
untracked mob walking across open ground. Vanilla pays per mob per tick:
4 attribute reads, the friction block read (a PalettedContainer walk),
the moveRelative, the FULL collide (block shape gathering + hard-collider
queries + the step-up branch) and the flag bookkeeping. The port:

1. **The delta math runs natively** (bit-exact f32/f64, pinned by
   `centities-core/src/mobtravel.rs`): computeModifiedFriction →
   getFrictionInfluencedSpeed (the classic 0.216/f³ path) → getInputVector
   (with the vanilla 65536-entry Mth sin LUT reproduced bit-exactly —
   `cos(90°)` through the table is ~9.59e-5, NOT 0) → the drag tail.
2. **The collide is skipped by proof**: the swept hull (from-box ∪ to-box,
   the ±1e-7 superset margins) is all air; the ground layer under the UNION
   footprint is full zero-bounce blocks; no hard colliders in the swept
   box; the world-border margin holds. `maybeBackOffFromEdge` must be the
   identity (compared by reference).
3. **The writeback reproduces the vanilla order** (javap of
   Entity.move, bc 516-815): `setPos(position + clipped)`; `recordMovement`
   (the real virtual call — the interpolation handler's
   applyPredictedMovement); `setOnGroundWithMovement(true, false, clipped)`
   — the REAL Level.findSupportingBlock rescan, exact by construction;
   `checkFallDamage(0.0, true, ..)` is a proven no-op (fallDistance == 0,
   moved dy == 0.0); the drag tail writes the final delta.

## The three findings worth keeping

1. **The restitution zeroes the delta.y.** `restituteMovementAfterCollisions`
   runs on the verticalCollision the ground clip just produced, and for a
   zero-bounce entity rewrites the delta as (x, 0.0, z). The travel tail's
   `d = dm.y - gravity` therefore starts from 0.0, and the post-tail
   delta.y = -gravity * computeModifiedFriction(0.98, airDrag) — the
   steady-walker fixed point the SHAPE B gate already pinned.
2. **`addMovementThisTick` is intentionally NOT reproduced.** For mobs the
   movementThisTick deque is consumed only by `applyEffectsFromBlocks`,
   which the L22b block-fx gate no-ops on the SAME all-air hull — the
   record is unobservable. Skipping it also keeps that gate armed (a
   non-empty deque disarms it: the gate bails to vanilla when the deque is
   non-empty).
3. **The routing keeps every observable vanilla wherever a player can see
   it**: untracked only (via `TrackedEntity.moonrise$hasPlayers()`), and
   the 0.2 powder-snow/climb branch is proven unable to fire (open hull ⇒
   no climbable contact; wasInPowderSnow required false; horizontal
   Collision required false) so the protected `jumping` field needs no
   read. Protected members (`maybeBackOffFromEdge`, `getEntityBounciness`)
   ride setAccessible MethodHandles — exact virtual dispatch.

## The e2e crash postmortem (hs_err 4006, fixed at 99cb4fa)

The first SHAPE C commit crashed the JVM at the first AI zombie spawn:
`SIGSEGV in C1 mobTravelNative+0x9cf`. Root cause:
`e.moonrise$getTrackedEntity()` returns **ChunkMap$TrackedEntity**, not
ServerEntity — the guard read `ServerEntity.trackedPlayers` by raw offset
against a TrackedEntity instance, got a garbage "oop", and the following
`instanceof Set` dereferenced it. This is the SAME wild-oop bug class that
killed the L19/20b SE_FAST path (a raw Unsafe read whose offset belongs to
a different class produces a fake reference that both crashes on first use
and poisons the GC — matching the flaky G1SATBMarkQueueSet::filter
signature). The fix reads `moonrise$hasPlayers()` (typed, public, and
exact: the TrackedEntity IS the ServerEntity$Synchronizer and seenBy is
the recipient set). Rule going forward: **every Unsafe/RefField offset is
single-class — a receiver of another class routes vanilla through a typed
check first.**

## Expected effect

The travel/collide bucket was 7.5% + the intersects 5.2% of the mob
profile (doc 30); SHAPE C removes the collide and the travel math for
open-ground walkers (the bench's wandering zombies), the friction block
read and 2 of the 4 attribute reads per covered tick. Watch the next
ab-paired run's mobs-25k and mixed legs.

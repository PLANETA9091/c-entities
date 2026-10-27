# 28. L20 design — the quiet-mob movement floor (the mobs endgame)

Date: 2026-10-25. Status: DESIGN (the next session-scale port, same shape
as the L6 item engine). After L16-L19 the mobs-25k-mod leg (149.9ms,
-58%) is dominated by the vanilla per-mob body: travel/collide (the
single largest slice — AABB.intersects + PalettedContainer.get + the
attribute reads + the collision glue), goals, navigation, senses, data.

## Where the remaining mobs time is (L18 profile, b0a2095/8c7cd00)

- travelInAir chain per mob per tick: `getBlockPosBelowThatAffectsMyMovement`
  + `getBlockState().getBlock().getFriction()` (a PalettedContainer walk),
  3-5 `AttributeMap.getValue` reads (FRICTION_MODIFIER, MOVEMENT_SPEED,
  AIR_DRAG_MODIFIER, GRAVITY via getEffectiveGravity), `moveRelative`
  (f64 math), `move(MoverType.SELF, delta)` = moonrise performCollisions
  (the AABB.intersects + shape queries), `onGround/fallDistance` updates.
- goals/navigation/look/sensing: already floored where legal; the goal
  OBJECT calls (canUse/canContinueToUse/tick) stay Java by design.
- baseTick block effects (applyEffectsFromBlocks + InsideBlockEffectApplier
  ≈ 2%), tickEffects, data reads.

## The port scope (exact-parity routing)

Route native ONLY when every uncertain input is provably boring, mirroring
the L6 graded-ceiling contract:

- on ground (or falling with no climbable contact), not in water/lava,
  `shouldTravelInFluid == false` (fluid check via the section cache's
  fluid-bearing flag), not fall-flying, no levitation/slow-falling effects
  (effect list read — route vanilla when the mob has ANY mob effects),
  no passengers/vehicle, no leash, not `shouldDiscardFriction()`,
  omnidirectionalAirMover false, not freezing (`ticksFrozen == 0` and no
  powder-snow contact), `hasEntityMoveEvent == false` (Paper config —
  already cached in PUSH_CFG-style per-tick config), section cache says
  the mob's box spans only full-cube sections (the L6 cache, extended
  with a per-section `stepOnOverride` + `hasFluid` flag).

The Rust body (centities-core/src/mobtick.rs — new):

1. `blockFriction = computeModifiedFriction(block.getFriction(), (float)
   attrValue(FRICTION_MODIFIER))` — the block friction comes from the
   section cache palette entry (cache the f32 per block type at
   cache-fill; the attribute values are synced from Java per flush).
2. `moveRelative(getFrictionInfluencedSpeed(blockFriction), input)` —
   the f64 port (`getFrictionInfluencedSpeed` = onGround ? speed*0.216
   pow path : speed*0.02 — EXACT f32/f64 semantics; input = (xxa, yya,
   zza) read fresh).
3. `handleOnClimbable` — route vanilla when `onClimbable()` possible
   (section flag: any climbable block in the box span).
4. `move(SELF, delta)` — the L6 `perform_collisions` port, EXTENDED with
   the step-up branch of `Entity.collide` (maxUpStep attribute, the
   second collide pass with the upward offset, the collided-axis
   fallback comparisons). The item engine already ported the axis order,
   epsilons (1e-7), the -0.5 ground branch and the moonrise single-AABB
   fast path.
5. post-move: onGround, fallDistance (f32), horizontalCollision/vertical
   collision flags, `setPos` writeback (Java applies at flush — the
   probe hooks keep the index exact), landing `playSound`/fall-damage
   events ride the flush's event batch (vanilla Java snippets, in
   vanilla body order — the L6 flush contract).
6. the friction/drag apply (`0.91/0.98 × computeModifiedFriction(...)`)
   + gravity (`getEffectiveGravity` — the gravity attribute + formula).

## State sync (the flush pattern)

ONE JNI crossing per tick per world (rides the existing item flush or
its sibling): Java passes (id, xxa, yya, zza, blockFriction-cache id,
attribute values (speed, friction-mod, air-drag, gravity, step),
yaw/pitch (moveRelative needs yaw), flags) — Rust computes the whole
travel slice → writeback: delta (DELTA_SAME mask), pos (MOVED),
onGround, fallDistance, collision flags, events (land, step sounds are
audible-only — players nearby route vanilla anyway via the tracker gate).

Attribute cache: refreshed by a probe on AttributeInstance value changes
(the setter is a few call sites; probe `AttributeInstance.setBaseValue/
addPermanentModifier` paths — OR refresh per flush (4 double reads per
mob per flush ≈ 25k × 4 × 20ns = 2ms — acceptable, no new weaves).

## Routing fallback

ANY doubt → `vanilla_next` (the item pattern): the whole travel slice
runs Java that tick. The 256-tick resync cadence belt from L6 applies.

## Expected ceiling

mobs-25k: the travel/collide slice is the largest per-mob cost; removing
it for quiet grounded mobs leaves ~goal+nav+sense+baseTick+dispatch —
est. -75..-85% total. The remaining tail needs the goal-object batch
(canUse calls for the wander/stay set) — a LATER, riskier move.

## Not in this milestone

- Fluid travel (route vanilla), fall-flying, ridden travel, climbing.
- Navigation/pathfollowing (stays Java; the pathfinder is out of scope).
- The `Entity.move` move-effect chain for non-full-cube sections.

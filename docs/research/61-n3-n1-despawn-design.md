# 61 — N3-N1 design: the native despawn decision (the M2c honesty return)

Date: 2026-11-12. Status: DESIGN (ground truth javap'd from the pinned
paper-26.3-218 jar; implementation pending). Doc 59 traded the native's
`checkDespawn` for ~3% of the server thread and left the debt open: a
resident native never hard-despawns. This document pins the EXACT
semantics the native decision must reproduce and the one validation gap
that blocks an honest ship.

## The vanilla body (Mob.checkDespawn, 26.3 bytecode)

```
1.  if difficulty == PEACEFUL &&
        despawnInPeacefulOverride.toBooleanOrElse(!type.isAllowedInPeaceful())
            -> discard(DESPAWN); return
2.  persistent = isPersistenceRequired() || requiresCustomPersistence()
3.  player = level.findNearbyPlayer(this, -1.0, PLAYER_AFFECTS_SPAWNING)
    if player == null -> return                 // -1.0 = unlimited range
4.  pair  = paperConfig.entities.spawning.despawnRanges[category]
    shape = paperConfig.entities.spawning.despawnRangeShape
5.  dy = |player.y - mob.y|; dy2 = dy*dy; dx2, dz2 likewise
    dist2 = dx2 + dz2 + dy2
6.  if !persistent && pair.hard().shouldDespawn(shape, dx2, dy2, dz2, dy)
            && removeWhenFarAway(dist2) -> discard(DESPAWN); return
7.  if pair.soft().shouldDespawn(shape, dx2, dy2, dz2, dy) {
        if !persistent && noActionTime > 600 && random.nextInt(800) == 0
                && removeWhenFarAway(dist2) -> discard(DESPAWN)
    } else {
        noActionTime = 0                        // the in-range RESET
    }
```

Everything is a pure per-mob function of (config, players, mob state)
EXCEPT two virtual calls (`requiresCustomPersistence()`,
`removeWhenFarAway(double)`) and one RNG draw.

## The native pieces

1. **The config table push (wave13 pattern)**: `despawnRanges`
   (hard/soft per MobCategory), `despawnRangeShape` and the difficulty +
   peaceful-override defaults cross to Rust ONCE at arm time
   (`nativeDespawnCfg`); `DespawnRange.shouldDespawn` is a pure shape
   function — port to Rust and differential-test against the Java
   oracle (the simd_parity shape).
2. **The predicate bits**: the wire's player push already computes
   `isAlive && !isSpectator`; PLAYER_AFFECTS_SPAWNING needs the exact
   predicate (gamemode + the Paper affectsSpawning flag) — one bit per
   player per boundary, computed in Java.
3. **The two virtuals**: `requiresCustomPersistence()` and
   `removeWhenFarAway()` are plugin-overridable. Per-tick JNI virtual
   calls are exactly the cost M2c deleted. Cache per CLASS (resolved
   through one JNI callback per class at classification; the shipped
   vanilla classes answer statically), re-resolve on the sight of a new
   class; per-INSTANCE overrides re-resolve lazily — a divergence
   window a plugin can only hit by swapping the override mid-life, an
   accepted ledger line (the M2c ledger precedent).
4. **The RNG draw**: `random.nextInt(800)` draws from
   `Entity.SHARED_RANDOM` — the global stream. The wave13/14 theorem
   survives: the draw COUNT per mob per tick is
   f(soft-range state, noActionTime>600, persistent) — computable
   BEFORE the draw, so the executor's prefix assignment covers it. The
   vanilla draw ORDER is the walk order (checkDespawn in the walk
   lambda, before the dispatcher) — the executor's serial pass must
   slot the despawn draws at the walk-order position, which the M2e
   route made the store order's twin (the route preserves the walk
   visit sequence).
5. **The outbox**: `nativeMobDiscard(eid, cause)` — the boundary
   applies `mob.discard(EntityRemoveEvent.Cause.DESPAWN)` on the tick
   thread; the removal driver (isRemoved) unregisters on the next
   boundary. `noActionTime = 0` rides the existing noAct mirror (the
   applier already writes NM_IDS[2i+1]).

## The validation gap (the honest blocker)

The parity harness has NO player (PARITY legs skip the bot) —
`findNearbyPlayer` returns null and the whole decision dies at step 3.
The bench legs are persistenceRequired — despawn never fires. TODAY NO
scenario in the repo can see a native despawn. The gate must be a NEW
harness scenario: player-present, NON-persistent mobs, spawn strata
inside/outside the hard and soft ranges (the 128-block instant line and
the soft-rate band), fixed seeds, snapshot count + position hashes per
stratum. Without it the Rust `shouldDespawn` port ships untested — the
doc 36 lesson (the harness catches what unit tests miss) forbids that.

## Cost model

The decision is O(players) per mob per tick in the native executor
(the soft-range compare + the maybe-draw), all inside the existing
pack pass — no Java-side per-mob work returns. The M2c savings stay;
the Java-side cost of the outbox is one JNI crossing per despawn EVENT
(rare by definition).

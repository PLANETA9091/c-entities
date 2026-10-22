# 18 — L7: dispatch takeover (the core owns the whole item dispatcher)

Owner directive (binding): the item tick is COMPUTED in Rust, never skipped.
L7 keeps that contract and removes the last Java per-entity floor around it.

## The problem after L6

L6 computes the tick body natively (the woven `ItemEntity.tick` guard), but
the vanilla **dispatcher** still ran for every item every tick:

`ServerLevel.tickNonPassenger(Entity)` (Paper 1.21.8-60, javap-verified):

```
TickThread.ensureTickThread
currentlyTickingEntity.lazySet
entity.setOldPosAndRot()            <- ~5% of tick CPU at 100k items
Profiler push x2                    <- one allocates a capturing lambda/entity
entity.tickCount++; totalEntityAge++
ActivationRange.checkIfActive(entity)  <- entity-vs-player AABB work
if (active) { entity.tick(); entity.postTick(); }   // postTick = handlePortal
else entity.inactiveTick()
for (passengers) tickPassenger
```

Plus the L6 collectors themselves (14 doubles + 4 ints per item per tick)
and the flush applier (setDeltaMovement allocs, mode table, merge, despawn).

## The lever: checkIfActive forces 25% of items through the full tick

`ActivationRange.checkIfActive` (io.papermc.paper.entity.activation,
javap-verified) returns **true unconditionally for ItemEntity whenever
`(tickCount + getId()) % 4 == 0`** — the merge-cadence rotation. So every
item full-ticks every 4th tick regardless of player distance: ~25% of a
100k-item world runs the complete vanilla tick every tick. Items otherwise
activate by `activatedTick >= currentTick` (player proximity) with a 20-tick
temporary wake whose immunity probe for items reduces to: fire → 2,
immunity stamp → 1, water+fluid-push → 100, airborne → 100,
on-ground-and-moving → 100, resting → temporarily active 1 tick.

## Design

### 1. The dispatch guard

An `EarlyReturnGuard` on `tickNonPassenger(Entity)V` calling
`CeHooks.itemDispatch(Entity)Z`. The SDK op-4 weaver (CRUSSTY a34d529) now
selects the predicate receiver from the method descriptor: first param is a
reference (`L...;`/`[`) → local 1 (the entity), else local 0 (this) —
unchanged for the existing no-arg guards.

`itemDispatch` suppresses the dispatcher when, in order:

1. `ACTIVE/ITEM_TICK_ON/cplugReady` and the entity is an `ItemEntity`;
2. MODE native (`MODE_GEN == epoch && MODE_VAL != 0`);
3. cheap field gates: removed / noPhysics / `portalProcess != null` /
   `portalCooldown > 0` / empty stack / has passengers → vanilla;
4. `MODE_REV[slot] != WORLD_REV` → the expensive L6 fresh-flag gates
   (`isInWater/isInLava/fireTicks/despawnTime`) run for that tick;
5. `itemCheckActive` (byte-faithful checkIfActive port for items, side
   effects included; tickCount reads are `field + 1` because the guard runs
   pre-increment) → append `IF_NATIVE` (active) or `IF_NATIVE|IF_INACTIVE`
   (inactive) and return true.

Anything uncertain returns false → the vanilla dispatcher runs and the L6
tick guard collects exactly as before (graceful degradation to L6 if this
weave is refused).

### 2. Rust-owned state

`ItemEntry` now carries the full state (x/y/z, delta, age, pickupDelay,
tickCount, despawnRate, fallDist, onGround + the L6 stuck/sup/friction
parts). For `IF_NATIVE` entries the batch carries only id+flags (plus the
guard's fresh `pickupDelay` read for plugin parity — a mid-flight change
wins). The core self-increments tickCount (the suppressed dispatcher's
`++`), computes `tick_item` / `tick_item_inactive`, commits, and the applier
mirrors everything back into the entity.

ItemWorlds are lazily allocated (8 × ~20 MB preallocation removed — the
100k bench legs died with runner shutdowns at exactly this kind of resident
overhead).

### 3. External mutations

- `probeBox` (setBoundingBox — the single box-change choke point) and
  `probeDelta` (setDeltaMovement) call `l7ExternalMutation`: any box/velocity
  change **outside** the applier's `IN_ITEM_APPLY` window clears the item's
  native mode → next dispatch routes vanilla → the L6 guard re-ingests.
  Pistons, explosions, plugin teleports all funnel through these two probes.
- `WORLD_REV` (bumped by the flush whenever any section changed since the
  previous flush) gates the expensive fluid/fire checks: in steady state
  (no block edits) they cost nothing; on a block-change tick the water that
  flowed onto a resting item routes it vanilla **on the very tick**.
- The 256-tick cadence belt (core-side): every 256th tick the item is flagged
  `VANILLA_NEXT`, runs one full vanilla tick + re-ingest. Heals any residual
  drift window. Dispatch-inactive items are re-collected by the inactiveTick
  guard for exactly that (GEN==EPOCH && VAL==0 identifies them).
- Abort streak: 4 consecutive native aborts force a vanilla tick — a
  permanently-imprecise item cannot freeze forever.

### 4. The applier mirrors the dispatcher

For suppressed dispatches the applier writes `tickCount = outI[i*4+3]` and
`totalEntityAge += 1` (outI layout is now `[age, pickupDelay, bits,
tickCount]`), stamps `MODE_REV[slot] = revBefore` (the revision current
before this flush's drain — any later bump forces the gates), and the
despawn rate check uses the live field for IF_NATIVE entries.

## What Java still does per native item per tick

The guard: instanceof + MODE table reads + ~6 field reads + the activation
port + one batch append (~40–70 ns). The applier: delta/age/pickupDelay/
fallDist writes + setPos-if-moved + counters (~60–100 ns). Everything else —
setOldPosAndRot, profiler, checkIfActive AABB, tick()/inactiveTick entry,
postTick, passengers, the L6 collectors — is gone.

## Verification

- 9 core trace/property tests green (tick semantics unchanged).
- Local boot smoke: dispatch guard precomputed + applied, ARMED, 0
  VerifyErrors; `dispChecks` climbing, flush prefix exactly 3 levels/tick.
- e2e now asserts both dispatch-weave log lines.
- A/B: the paired items-100k / spread-100k / mobs-25k / mixed-150k legs carry
  the verdict (CI runner flakiness on ubuntu-26 mitigated: max-parallel 6,
  heap 4G, JFR maxsize, per-leg workflow_dispatch reruns).

## Next levers (L8+)

- The remaining floor: `EntityTickList.forEach` iteration, tracker
  (`ChunkMap$TrackedEntity.updatePlayer`), `NaturalSpawner.createState`,
  activation pre-pass — each is a separate weave target.
- Rayon-parallel flush for the item core (the store shards cleanly by id).

# How entity ticking works in Paper 1.21.8 — facts from the sources

Source: the real `PaperMC/Paper` code, branch `ver/1.21.8` (patches on top
of decompiled vanilla 1.21.8, mojang mappings). Everything below is
confirmed by reading the code — no guesswork.

## 1. World tick loop (ServerLevel.tick)

Every tick, on a single thread:

1. Weather, time, natural spawning, chunks (light, random ticks).
2. **`ActivationRange.activateEntities(level)`** — Paper patch, see §3.
3. **`entityTickList.forEach(entity -> ...)`** — iterates **ALL** entities
   in loaded chunks (not just near players!):
   ```java
   this.entityTickList.forEach(entity -> {
       if (!entity.isRemoved()) {
           if (tickRateManager... /* vanilla tick-rate checks */) {
               this.tickNonPassenger(entity);
           }
       }
   });
   ```
4. Block entities: `tickBlockEntities()` — see §5.

## 2. tickNonPassenger — what happens per entity per tick

```java
public void tickNonPassenger(Entity entity) {
    entity.setOldPosAndRot();
    entity.tickCount++;
    profilerFiller.push(() -> ENTITY_TYPE.getKey(entity.getType()).toString()); // profiler is not free
    final boolean isActive = ActivationRange.checkIfActive(entity); // Paper EAR 2
    if (isActive) {
        entity.tick();          // full vanilla tick
        entity.postTick();      // CraftBukkit (events + tail logic)
    } else {
        entity.inactiveTick();  // cheap alternative tick
    }
    // + recursive tickPassenger for passengers
}
```

Key point: **a virtual call + profiler push for every entity every tick**,
even for entities that are immediately dropped by activation.

## 3. Entity Activation Range 2.0 (Paper)

### activateEntities — the activation scan
Every tick, for EVERY player:
```java
final List<Entity> entities = world.getEntities((Entity) null, ActivationRange.maxBB, e -> true);
for (final Entity entity : entities) ActivationRange.activateEntity(entity);
```
That is **O(N_entities) per player per tick** — a full world-wide AABB
query. With 100k+ entities the "activator" alone is a measurable cost
(the benchmark will confirm).

Always-active types (defaultActivationState = true): Player, EyeOfEnder,
ThrowableProjectile, EnderDragon, WitherBoss, AbstractHurtingProjectile,
LightningBolt, PrimedTnt, FallingBlock, Minecart, Boat, EndCrystal,
FireworkRocket, ThrownTrident. Markers — per `entities.markers.tick` config.

### checkIfActive — the "tick or not" decision
- FireworkRocket and ItemEntity on every 4th tick
  (`(tickCount + id) % 4 == 0`) — **always active** (item gravity is
  never turned off).
- Fresh entities (< 200 ticks), portal states, leashed-to-player — active.
- `entity.activatedTick >= currentTick` → active.
- Inactive entities: every 20 ticks an "immunity" check
  (`checkEntityImmunities`): fire, water/swimming, `!onGround`,
  horizontal speed > 1e-5, arrow not in ground, climbing, jumping,
  effects, `mob.getTarget() != null`, babies/love, ignited creeper, etc.
  Immunity grants an activity window of N ticks.
- Otherwise — `inactiveTick()`: mobs skip goals/navigation/travel,
  items skip almost everything.

### EAR takeaways
EAR is **already a tiered scheduler shipped with Paper**, but:
1. The activation scan is O(N) per player.
2. `checkIfActive` is a branch-heavy check on every entity.
3. Immunities are re-checked blindly every 20 ticks.
4. Item physics still ticks (every 4th tick is mandatory).

## 4. Physics of specific types

### ItemEntity.tick (items)
- Gravity: **0.04**/tick; friction: **0.98** (in air), on ground
  `blockFriction * 0.98`; `move(MoverType.SELF, delta)` → full
  collision sweep against blocks every tick while moving, or every
  4th tick otherwise
  (`(!onGround || hSpeed^2 > 1e-5 || (tickCount+id)%4==0)`).
- Water: buoyancy (upward drift).
- **Merging — every tick for a mergeable item:**
  ```java
  getEntitiesOfClass(ItemEntity.class, getBoundingBox().inflate(radius, ..., radius), ...)
  ```
  where `radius = spigotConfig.itemMerge` (default 0.5).
  This is an **AABB query over world entities per item per tick** —
  O(N)-ish behavior without a spatial index. With 100k items it is the
  prime avalanche candidate (to be confirmed by the JFR profile).
- Despawn: `age >= 6000` (spigot despawn-rate).
- PickupDelay 32767 = "never pickable"; such items never merge
  (`isMergable()` false) and skip the merge scan.

### LivingEntity (mobs)
tick() order: baseTick (fire/lava/void) → `aiStep()` (goals/brain →
input) → `serverAiStep()` → `travel()` (friction, gravity 0.08 for
living entities, moveRelative) → `move()` → **AABB collisions against
blocks** + pushing other entities (`getEntities` around the bounding
box; Paper caps it via `max-entity-collisions`, default 8).
On top: goalSelector/targetSelector tick every tick for active mobs.

### ExperienceOrb
Searches for a player within attraction radius every tick, gravity 0.03,
merging similar to items. EAR gives it a 20-tick window.

## 5. Block entities (Level.tickBlockEntities)

```java
// blockEntityTickers is a plain ArrayList for the whole level
for (this.tileTickPosition = 0; tileTickPosition < blockEntityTickers.size(); tileTickPosition++) {
    TickingBlockEntity tickingBlockEntity = blockEntityTickers.get(tileTickPosition);
    if (tickingBlockEntity.isRemoved()) { toRemove.add(...); }
    else if (runsNormally && this.shouldTickBlocksAt(tickingBlockEntity.getPos())) {
        tickingBlockEntity.tick();   // virtual call per BE per tick
    }
}
```

- **Full iteration of the whole list every tick** + a per-BE chunk check
  (`shouldTickBlocksAt`).
- **Furnace** (AbstractFurnaceBlockEntity.serverTick): `isLit` check,
  `canBurn` (recipe check via per-type cached `RecipeManager.createCheck`),
  `cookingTimer++` every tick while cooking, result placement, Bukkit event.
- **Hopper** (HopperBlockEntity): `tryMoveItems` every tick — pulls from
  the inventory above, pushes down/sideways; cooldown of 8 ticks after a
  successful transfer; Paper exposes a hopper-cooldown config.
- 100k block entities = 100k virtual calls + chunk checks every tick.

## 6. Suspected hot spots at 100k+ (to be verified by the benchmark)

| # | Suspect | Where | Why |
|---|---------|-------|-----|
| 1 | Item merge scans | ItemEntity.tick | O(N) AABB query per item per tick |
| 2 | Activation scan | EAR activateEntities | O(N) per player per tick |
| 3 | checkIfActive | EAR | branch-heavy logic per entity |
| 4 | Block collisions | Entity.move | sweep per moving entity |
| 5 | Virtual tick() dispatch | tickNonPassenger | dispatch + profiler push ×N |
| 6 | Entity pushing | LivingEntity.push | getEntities around the box |
| 7 | BE iteration | tickBlockEntities | N virtual calls + chunk check |
| 8 | GC pressure | getEntities / AABB queries | per-tick list allocations |

Each entry becomes a measured fact after the baseline run (JFR top frames).

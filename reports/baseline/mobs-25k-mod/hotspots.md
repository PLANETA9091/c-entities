# JVM hotspots (JFR ExecutionSample)

Samples: 15217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.62% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1892` |
| 2 | 4.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3746` |
| 3 | 2.60% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 1.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3467` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1893` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3729` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3747` |
| 14 | 1.05% | `java.util.ArrayList.add(Object) line: 497` |
| 15 | 1.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10126` |
| 16 | 0.99% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 17 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6732` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9779` |
| 20 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.76% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 23 | 0.75% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10204` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10749` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9957` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6669` |
| 30 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.60% | `java.util.ArrayList.add(Object) line: 496` |
| 33 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.51% | `java.util.ArrayList.grow() line: 245` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 39 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 40 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3814` |
| 42 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.42% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 44 | 0.42% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6978` |
| 47 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 50 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3761` |
| 52 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 55 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7046` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 15410` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13849` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.77% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 8.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3467` |
| 3 | 3.67% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10219` |
| 5 | 2.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10204` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3815` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.55% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 12 | 1.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 13 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1884` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10125` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 0.91% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10295` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6979` |
| 22 | 0.78% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.77% | `java.lang.ThreadLocal.get() line: 171` |
| 24 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.61% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.51% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |

GC pauses: 0, total 0 ms


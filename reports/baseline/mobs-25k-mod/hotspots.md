# JVM hotspots (JFR ExecutionSample)

Samples: 17977

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.05% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1842` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3686` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3685` |
| 4 | 1.99% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 1.54% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 6 | 1.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3406` |
| 7 | 1.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3753` |
| 9 | 1.25% | `java.util.ArrayList.add(Object) line: 497` |
| 10 | 1.16% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 11 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.82% | `java.util.ArrayList.elementData(int) line: 412` |
| 16 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1843` |
| 18 | 0.72% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3668` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5169` |
| 26 | 0.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 28 | 0.55% | `java.util.ArrayList.grow() line: 245` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6595` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9625` |
| 31 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 32 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9972` |
| 33 | 0.47% | `java.util.ArrayList.add(Object) line: 496` |
| 34 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 36 | 0.43% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1841` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6532` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13695` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 40 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3700` |
| 42 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9803` |
| 45 | 0.34% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6841` |
| 47 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 51 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 52 | 0.29% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 55 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 15192` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10595` |
| 57 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.27% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 60 | 0.27% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.24% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 9.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3406` |
| 3 | 4.24% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3754` |
| 5 | 1.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1834` |
| 8 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.17% | `java.lang.ThreadLocal.get() line: 171` |
| 10 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10065` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10050` |
| 15 | 0.96% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 16 | 0.87% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.82% | `java.util.ArrayList.get(int) line: 429` |
| 20 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14711` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9971` |
| 23 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.64% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.56% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.55% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 29 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10141` |

GC pauses: 0, total 0 ms


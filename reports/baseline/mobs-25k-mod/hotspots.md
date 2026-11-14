# JVM hotspots (JFR ExecutionSample)

Samples: 20813

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.94% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 3.02% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 3 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.17% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 7 | 1.11% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 11 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8762` |
| 12 | 0.98% | `java.util.ArrayList.elementData(int) line: 412` |
| 13 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8840` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8593` |
| 16 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 18 | 0.76% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12886` |
| 20 | 0.67% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8415` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 23 | 0.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 25 | 0.57% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5822` |
| 27 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 28 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5885` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.47% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 32 | 0.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9385` |
| 34 | 0.44% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8442` |
| 35 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.43% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 38 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 41 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3021` |
| 43 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.34% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 46 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 47 | 0.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 858` |
| 48 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.31% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 51 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3108` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6214` |
| 56 | 0.28% | `java.util.EnumMap.get(Object) line: 248` |
| 57 | 0.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.27% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13503` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 3.94% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 4 | 2.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.33% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 2.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8855` |
| 7 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 10 | 1.36% | `java.util.ArrayList.add(Object) line: 497` |
| 11 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8840` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8931` |
| 16 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 0.98% | `java.util.ArrayList.get(int) line: 429` |
| 18 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13128` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8761` |
| 20 | 0.83% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.76% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 23 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.62% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8577` |
| 27 | 0.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 29 | 0.54% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


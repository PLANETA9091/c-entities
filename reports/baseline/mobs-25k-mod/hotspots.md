# JVM hotspots (JFR ExecutionSample)

Samples: 100160

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 44.82% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 7.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 3 | 5.04% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 4.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 5 | 3.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 6 | 2.12% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2699` |
| 8 | 1.40% | `java.util.ArrayList.add(Object) line: 497` |
| 9 | 1.25% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 10 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2885` |
| 11 | 0.98% | `java.util.ArrayList.add(Object) line: 496` |
| 12 | 0.90% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 13 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2927` |
| 14 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2893` |
| 15 | 0.63% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 16 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 17 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 18 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2962` |
| 20 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2909` |
| 21 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.40% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2790` |
| 24 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2878` |
| 25 | 0.30% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2876` |
| 27 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.26% | `java.util.ArrayList.grow() line: 245` |
| 29 | 0.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 30 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.21% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1379` |
| 34 | 0.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7904` |
| 35 | 0.19% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 36 | 0.18% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.16% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7735` |
| 39 | 0.15% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 40 | 0.15% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7557` |
| 41 | 0.14% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7982` |
| 42 | 0.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 43 | 0.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 45 | 0.12% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 46 | 0.12% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 47 | 0.12% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7584` |
| 48 | 0.11% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.11% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.11% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 51 | 0.11% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 53 | 0.10% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5610` |
| 54 | 0.10% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5218` |
| 55 | 0.10% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 56 | 0.10% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.10% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 58 | 0.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.09% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1374` |
| 60 | 0.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 47.33% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 18.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 3 | 6.08% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 3.59% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1369` |
| 5 | 2.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2963` |
| 6 | 2.12% | `java.util.ArrayList.get(int) line: 429` |
| 7 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 8 | 0.57% | `java.util.ArrayList.grow(int) line: 238` |
| 9 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 11 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7997` |
| 12 | 0.35% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 0.30% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 14 | 0.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 0.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 16 | 0.26% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 17 | 0.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8073` |
| 22 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7903` |
| 24 | 0.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.18% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.18% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.17% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7719` |
| 29 | 0.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |

GC pauses: 0, total 0 ms


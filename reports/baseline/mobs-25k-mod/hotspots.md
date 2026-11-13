# JVM hotspots (JFR ExecutionSample)

Samples: 99479

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 43.69% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 2 | 7.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 3 | 4.71% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 4.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |
| 5 | 3.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2659` |
| 6 | 1.98% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2700` |
| 8 | 1.29% | `java.util.ArrayList.add(Object) line: 497` |
| 9 | 1.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 10 | 1.02% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 11 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1379` |
| 12 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2886` |
| 13 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2878` |
| 14 | 0.81% | `java.util.ArrayList.add(Object) line: 496` |
| 15 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2963` |
| 16 | 0.74% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 17 | 0.71% | `java.util.ArrayList.grow() line: 245` |
| 18 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2910` |
| 19 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 21 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 23 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.39% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 25 | 0.28% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 26 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2928` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2879` |
| 31 | 0.23% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.21% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 34 | 0.20% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1380` |
| 35 | 0.20% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 36 | 0.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 37 | 0.19% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 38 | 0.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2791` |
| 39 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 40 | 0.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 41 | 0.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7945` |
| 42 | 0.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8023` |
| 43 | 0.17% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 44 | 0.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.16% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7598` |
| 46 | 0.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 47 | 0.14% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7776` |
| 48 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 51 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.12% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 54 | 0.11% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11904` |
| 55 | 0.10% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 56 | 0.10% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5219` |
| 57 | 0.10% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7625` |
| 58 | 0.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.09% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.09% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 46.19% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 18.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2659` |
| 3 | 6.47% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 3.44% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1370` |
| 5 | 2.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2964` |
| 6 | 1.98% | `java.util.ArrayList.get(int) line: 429` |
| 7 | 0.71% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 8 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 9 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 0.51% | `java.util.ArrayList.grow(int) line: 238` |
| 11 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 12 | 0.39% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 13 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8038` |
| 14 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.38% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 17 | 0.28% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 18 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 20 | 0.24% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 21 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7944` |
| 24 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.21% | `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal.findTarget() line: 66` |
| 27 | 0.20% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 28 | 0.19% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 30 | 0.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8114` |

GC pauses: 0, total 0 ms


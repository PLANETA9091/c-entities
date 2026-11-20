# JVM hotspots (JFR ExecutionSample)

Samples: 51303

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 52.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3881` |
| 2 | 5.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3953` |
| 3 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3895` |
| 4 | 1.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 5 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3871` |
| 6 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3880` |
| 7 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3863` |
| 9 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 13 | 0.42% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 17 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3879` |
| 18 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9619` |
| 21 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14243` |
| 22 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10694` |
| 25 | 0.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10772` |
| 26 | 0.31% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 27 | 0.29% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 28 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 31 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3733` |
| 32 | 0.26% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10525` |
| 33 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10347` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10374` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2015` |
| 39 | 0.22% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 41 | 0.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 42 | 0.20% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.20% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7246` |
| 45 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 46 | 0.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 47 | 0.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3862` |
| 50 | 0.16% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.16% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.15% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 53 | 0.14% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7243` |
| 54 | 0.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 55 | 0.14% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.14% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11317` |
| 58 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.12% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 62.30% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2010` |
| 2 | 1.22% | `java.util.ArrayList.grow(int) line: 238` |
| 3 | 0.96% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10787` |
| 6 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10772` |
| 12 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.42% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 14 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 15 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10863` |
| 17 | 0.35% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10693` |
| 19 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 20 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.34% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 23 | 0.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.31% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 26 | 0.30% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.29% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 29 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10509` |

GC pauses: 0, total 0 ms


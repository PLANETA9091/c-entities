# JVM hotspots (JFR ExecutionSample)

Samples: 17897

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.02% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 4.17% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 2.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3126` |
| 4 | 2.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 5 | 2.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 6 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 9 | 0.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.87% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 12 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.79% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 14 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12828` |
| 16 | 0.74% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.65% | `java.lang.ThreadLocal.get(Thread) line: 185` |
| 19 | 0.61% | `java.util.ArrayList.add(Object) line: 497` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8704` |
| 21 | 0.59% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 22 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.58% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 26 | 0.55% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 27 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8357` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5758` |
| 31 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5821` |
| 32 | 0.47% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 33 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.38% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8535` |
| 37 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4591` |
| 39 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 40 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 42 | 0.33% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 43 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 45 | 0.32% | `java.util.ArrayList.add(Object) line: 496` |
| 46 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 47 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 48 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5733` |
| 51 | 0.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 52 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9327` |
| 54 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 56 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3021` |
| 57 | 0.27% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 59 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.68% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 7.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 3 | 4.17% | `java.util.ArrayList.get(int) line: 429` |
| 4 | 2.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 5 | 2.25% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.20% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13070` |
| 13 | 0.96% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8797` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 16 | 0.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8782` |
| 18 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8703` |
| 24 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.66% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.65% | `java.lang.ThreadLocal.get() line: 171` |
| 28 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.61% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms


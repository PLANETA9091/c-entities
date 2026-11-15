# JVM hotspots (JFR ExecutionSample)

Samples: 15546

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 4.29% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 2.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 4 | 2.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 5 | 2.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 6 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 11 | 0.88% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 0.87% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 13 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 14 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 16 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8879` |
| 19 | 0.66% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 20 | 0.64% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5853` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 23 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.60% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.59% | `java.util.ArrayList.add(Object) line: 497` |
| 26 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8710` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9502` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4608` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 32 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5916` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3210` |
| 34 | 0.46% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 35 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8532` |
| 38 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 40 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 44 | 0.38% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 45 | 0.37% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.35% | `java.util.ArrayList.grow() line: 245` |
| 48 | 0.35% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 50 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.33% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 52 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8957` |
| 54 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 56 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5828` |
| 59 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 60 | 0.24% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.83% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 3 | 4.28% | `java.util.ArrayList.get(int) line: 429` |
| 4 | 2.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 5 | 1.96% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8972` |
| 13 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 16 | 0.89% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.88% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1402` |
| 20 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8957` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |
| 24 | 0.76% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.68% | `java.lang.ThreadLocal.get() line: 171` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9048` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8878` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms


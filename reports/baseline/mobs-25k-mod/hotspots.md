# JVM hotspots (JFR ExecutionSample)

Samples: 21385

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.47% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 2.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 3 | 2.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 4 | 2.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 5 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.10% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 7 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12989` |
| 10 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.81% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.65% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3210` |
| 19 | 0.64% | `java.util.ArrayList.add(Object) line: 496` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8865` |
| 21 | 0.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 24 | 0.57% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8696` |
| 26 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.putAll(Map) line: 226` |
| 28 | 0.47% | `java.util.HashMap.getNode(Object) line: 586` |
| 29 | 0.46% | `java.util.ArrayList.elementData(int) line: 412` |
| 30 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 31 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5853` |
| 33 | 0.43% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8518` |
| 34 | 0.37% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 35 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 37 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3038` |
| 38 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9488` |
| 40 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 47 | 0.30% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 48 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5916` |
| 49 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6242` |
| 51 | 0.28% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 52 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.27% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 54 | 0.27% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5828` |
| 55 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 57 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.25% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13606` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3157` |
| 60 | 0.25% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.29% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 3 | 2.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 4 | 2.20% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 7 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.21% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13231` |
| 11 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8943` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8958` |
| 19 | 0.80% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |
| 23 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9034` |
| 25 | 0.65% | `java.util.ArrayList.grow(int) line: 238` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8864` |
| 27 | 0.59% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.51% | `java.util.HashMap.get(Object) line: 565` |

GC pauses: 0, total 0 ms


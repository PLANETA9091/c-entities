# JVM hotspots (JFR ExecutionSample)

Samples: 18367

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.90% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 3.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 4 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 13672` |
| 5 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 0.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 9 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12967` |
| 10 | 0.82% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 11 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.76% | `java.util.ArrayList.add(Object) line: 497` |
| 13 | 0.61% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 0.59% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.56% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 17 | 0.54% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 18 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3210` |
| 19 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4608` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8843` |
| 23 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 25 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.44% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5916` |
| 28 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8496` |
| 30 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.39% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8674` |
| 33 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 35 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 36 | 0.38% | `java.util.ArrayList.elementData(int) line: 412` |
| 37 | 0.36% | `java.util.EnumMap.get(Object) line: 248` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8299` |
| 39 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5853` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 42 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 48 | 0.28% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 49 | 0.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 50 | 0.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 51 | 0.27% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9466` |
| 52 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 53 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 57 | 0.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 59 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.25% | `java.util.ArrayList.add(Object) line: 496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.48% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 7.51% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 3 | 1.92% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 1.80% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 7 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13209` |
| 9 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |
| 10 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1402` |
| 16 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 17 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 0.78% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 19 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 0.62% | `java.util.HashMap.get(Object) line: 565` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8936` |
| 22 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8842` |
| 25 | 0.55% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9012` |
| 28 | 0.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 29 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.47% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |

GC pauses: 0, total 0 ms


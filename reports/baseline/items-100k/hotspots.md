# JVM hotspots (JFR ExecutionSample)

Samples: 12969

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 3.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 1.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.77% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 16 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 0.81% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.79% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 318` |
| 22 | 0.76% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 23 | 0.72% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 27 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 33 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 35 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 38 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 39 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.46% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 41 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.39% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |
| 46 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 53 | 0.32% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 54 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 57 | 0.29% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 213` |
| 58 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 60 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.58% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.99% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.73% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 16 | 1.30% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 17 | 1.15% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 18 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 1.01% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 1.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 22 | 0.97% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 23 | 0.93% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 24 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 26 | 0.80% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 27 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 30 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


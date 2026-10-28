# JVM hotspots (JFR ExecutionSample)

Samples: 15046

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.86% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 3.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.21% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.56% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 13 | 1.56% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.44% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 1.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 20 | 1.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 1.23% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 23 | 1.08% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 24 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 1.03% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 26 | 1.02% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.92% | `net.minecraft.world.entity.Entity.getYRot() line: 5275` |
| 29 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.77% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.72% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 32 | 0.68% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 35 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 37 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 38 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 41 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 44 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.41% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 46 | 0.41% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 306` |
| 47 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 48 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 52 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.30% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 59 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.99% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 5 | 3.12% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 6 | 3.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.52% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 9 | 2.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.56% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 14 | 1.56% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 15 | 1.51% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 1.15% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.setOldRot() line: 2400` |
| 23 | 0.94% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.84% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 27 | 0.78% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 28 | 0.77% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 29 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

GC pauses: 0, total 0 ms


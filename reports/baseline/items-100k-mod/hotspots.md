# JVM hotspots (JFR ExecutionSample)

Samples: 8907

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 4.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.74% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 2.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.86% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 10 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 11 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.39% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 16 | 1.07% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 17 | 1.01% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 18 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 19 | 0.92% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 20 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 21 | 0.89% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 22 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 23 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 24 | 0.74% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 27 | 0.60% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 28 | 0.58% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 30 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 31 | 0.52% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 32 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 34 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 36 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 40 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 41 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 42 | 0.40% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 43 | 0.39% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 45 | 0.38% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |
| 46 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 47 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 48 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.31% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 53 | 0.30% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 54 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 56 | 0.29% | `java.util.Objects.equals(Object, Object) line: 64` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 59 | 0.28% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 500` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 4.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 3.54% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 5 | 3.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.74% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 8 | 2.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 14 | 1.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 15 | 1.52% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.25% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 18 | 1.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 1.00% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 22 | 0.97% | `java.util.ArrayList.toArray() line: 369` |
| 23 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 25 | 0.91% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 28 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 29 | 0.76% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 30 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


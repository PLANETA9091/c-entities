# JVM hotspots (JFR ExecutionSample)

Samples: 8511

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 4.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.74% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 9 | 1.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 10 | 1.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 14 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.15% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 17 | 1.12% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 18 | 1.12% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 19 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 21 | 0.99% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.98% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 23 | 0.90% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 24 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 25 | 0.68% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 26 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 27 | 0.66% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 28 | 0.63% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 29 | 0.63% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 30 | 0.63% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 31 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 32 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 33 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 35 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 36 | 0.59% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 37 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 38 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 39 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 40 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 41 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 44 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 45 | 0.42% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 46 | 0.41% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 47 | 0.38% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 48 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 50 | 0.35% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 51 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 52 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 55 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 58 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 60 | 0.27% | `java.util.Objects.equals(Object, Object) line: 64` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 4.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 3.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.62% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 3.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 10 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.66% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 13 | 1.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 14 | 1.60% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 1.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 17 | 1.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 18 | 1.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 1.12% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 21 | 1.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 1.01% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.95% | `java.util.ArrayList.toArray() line: 369` |
| 25 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |
| 26 | 0.93% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 27 | 0.87% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 28 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.83% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |

GC pauses: 0, total 0 ms


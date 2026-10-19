# JVM hotspots (JFR ExecutionSample)

Samples: 3691

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 2.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 3 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.82% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 7 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.65% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 12 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.08% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 16 | 1.08% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 18 | 1.06% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 19 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 20 | 0.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 23 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 25 | 0.76% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 26 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 28 | 0.70% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 29 | 0.65% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 30 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 31 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 32 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 33 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 34 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 36 | 0.49% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 37 | 0.49% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 39 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.46% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 41 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 43 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 47 | 0.38% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 50 | 0.35% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 52 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 54 | 0.33% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 55 | 0.33% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 57 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 58 | 0.30% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 59 | 0.30% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 2.79% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 3 | 2.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 2.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 7 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.84% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 11 | 1.65% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.22% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 15 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 16 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.08% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 18 | 1.08% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 21 | 1.00% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 22 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 23 | 0.98% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 24 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 25 | 0.95% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 26 | 0.87% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.81% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 28 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 29 | 0.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |

GC pauses: 0, total 0 ms


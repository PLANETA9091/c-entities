# JVM hotspots (JFR ExecutionSample)

Samples: 3300

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 4.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.09% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 1.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 8 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 15 | 1.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 16 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 17 | 1.06% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 18 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 19 | 0.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 160` |
| 21 | 0.82% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 22 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 23 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 25 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 26 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 27 | 0.70% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.67% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 30 | 0.64% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 31 | 0.64% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 33 | 0.58% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 35 | 0.55% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.52% | `java.util.EnumMap.get(Object) line: 247` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 39 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 41 | 0.45% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 42 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.42% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 44 | 0.39% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 46 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 47 | 0.39% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 48 | 0.36% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 49 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 50 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 51 | 0.36% | `java.util.Arrays.fill(int[], int, int, int) line: 3204` |
| 52 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 54 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 55 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.30% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 57 | 0.30% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 58 | 0.30% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 59 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 2 | 5.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 4.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.09% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 9 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.48% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 14 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.27% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 17 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 18 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 19 | 1.06% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 20 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 21 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 24 | 0.82% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 25 | 0.82% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.79% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 28 | 0.76% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 29 | 0.76% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 30 | 0.76% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 3544

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.40% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 2 | 7.67% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.25% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 4.46% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.35% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.03% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 2.00% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.86% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 10 | 1.81% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 12 | 1.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.47% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 14 | 1.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 16 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.93% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 18 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 19 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.90% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 21 | 0.82% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 23 | 0.68% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 24 | 0.68% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.65% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 26 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 27 | 0.56% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 29 | 0.51% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 30 | 0.48% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 31 | 0.45% | `net.minecraft.world.level.block.entity.BlockEntity.getBlockPos() line: 244` |
| 32 | 0.42% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 33 | 0.40% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 34 | 0.40% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 35 | 0.37% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 36 | 0.34% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 185` |
| 37 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 38 | 0.34% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 39 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 40 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 41 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 42 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 43 | 0.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 44 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 45 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.28% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 47 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.25% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 49 | 0.25% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 50 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 51 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.23% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 53 | 0.23% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 54 | 0.23% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 55 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 56 | 0.23% | `java.util.concurrent.ConcurrentSkipListMap.pollFirstEntry() line: 2086` |
| 57 | 0.20% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 58 | 0.20% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 59 | 0.20% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 60 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.40% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 2 | 7.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 3 | 5.02% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 4.40% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 4.09% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 6 | 3.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 7 | 3.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 8 | 2.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.23% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 2.03% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 11 | 1.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.92% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 13 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 14 | 1.41% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 15 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 16 | 1.24% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 17 | 1.21% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 18 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.04% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 20 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 21 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.93% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 23 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 24 | 0.62% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 26 | 0.56% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 27 | 0.56% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 28 | 0.54% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 30 | 0.51% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |

GC pauses: 0, total 0 ms


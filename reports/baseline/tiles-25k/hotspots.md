# JVM hotspots (JFR ExecutionSample)

Samples: 1816

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.56% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.52% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 2.48% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 5 | 2.31% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.87% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 9 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 10 | 1.76% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 11 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.54% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.43% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.43% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 15 | 1.32% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 16 | 1.21% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 19 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 20 | 1.05% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 1.05% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 22 | 1.05% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 23 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 24 | 0.94% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 25 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 26 | 0.83% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 27 | 0.83% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 28 | 0.83% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 29 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 30 | 0.72% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 31 | 0.72% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 32 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.55% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 34 | 0.55% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.LevelChunkSection.getBlockState(int, int, int) line: 77` |
| 36 | 0.55% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 37 | 0.50% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 38 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 39 | 0.50% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 40 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 41 | 0.44% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 42 | 0.44% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 43 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 228` |
| 45 | 0.39% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 47 | 0.33% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 48 | 0.33% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 49 | 0.33% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 50 | 0.33% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 51 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 53 | 0.33% | `java.io.BufferedOutputStream.flushBuffer() line: 124` |
| 54 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 55 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 56 | 0.28% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 57 | 0.28% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getMinY() line: 482` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 60 | 0.28% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 2.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.48% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 5 | 2.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 6 | 2.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 7 | 1.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 8 | 1.98% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 9 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 10 | 1.76% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 11 | 1.60% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 12 | 1.54% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 13 | 1.54% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 14 | 1.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 15 | 1.43% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 16 | 1.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 17 | 1.43% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 18 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.10% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 20 | 1.05% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 21 | 1.05% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 22 | 0.99% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 23 | 0.94% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 24 | 0.88% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 25 | 0.83% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 26 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 27 | 0.83% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 28 | 0.77% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 29 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 30 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |

GC pauses: 0, total 0 ms


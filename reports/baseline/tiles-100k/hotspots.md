# JVM hotspots (JFR ExecutionSample)

Samples: 3037

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.32% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.47% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 3 | 4.87% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 4.25% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 3.72% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.96% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 7 | 2.60% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 2.47% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 2.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.88% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.58% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 15 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.22% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 17 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 18 | 1.05% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 19 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.76% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 23 | 0.69% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 24 | 0.66% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 25 | 0.66% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 381` |
| 27 | 0.66% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 29 | 0.56% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 30 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 31 | 0.53% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 32 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 33 | 0.49% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 34 | 0.46% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 35 | 0.46% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 36 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 527` |
| 37 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 38 | 0.40% | `java.lang.String.hashCode() line: 2358` |
| 39 | 0.40% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 40 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 41 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 147` |
| 42 | 0.36% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 43 | 0.36% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 44 | 0.36% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 45 | 0.33% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 46 | 0.33% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 48 | 0.33% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1460` |
| 49 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 50 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 51 | 0.26% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 52 | 0.26% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 53 | 0.26% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 54 | 0.26% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 55 | 0.26% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 56 | 0.26% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 85` |
| 57 | 0.23% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 58 | 0.23% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 59 | 0.23% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.noise(double, double, double, double, double) line: 61` |
| 60 | 0.23% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.66% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 3 | 2.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 4 | 2.57% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 5 | 2.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 2.50% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 7 | 2.37% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 8 | 2.24% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 9 | 2.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 10 | 2.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 11 | 2.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 2.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.94% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 14 | 1.58% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 15 | 1.55% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 16 | 1.55% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 17 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 18 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.22% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 20 | 1.12% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 21 | 1.05% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 22 | 0.95% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 23 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 24 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 25 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 26 | 0.72% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 27 | 0.66% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 28 | 0.66% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 29 | 0.63% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.63% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |

GC pauses: 0, total 0 ms


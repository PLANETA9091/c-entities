# JVM hotspots (JFR ExecutionSample)

Samples: 1937

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 2.63% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.48% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 4 | 2.27% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 2.01% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 8 | 1.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 9 | 1.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 12 | 1.24% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 13 | 1.14% | `java.lang.String.hashCode() line: 2358` |
| 14 | 1.14% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 15 | 1.08% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 16 | 1.03% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 17 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 18 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 19 | 0.88% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 21 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.77% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 23 | 0.77% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 24 | 0.72% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 25 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 27 | 0.67% | `java.util.HashMap.getNode(Object) line: 585` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 29 | 0.62% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 30 | 0.62% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 31 | 0.62% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 32 | 0.57% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 33 | 0.57% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 34 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 35 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 36 | 0.52% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 37 | 0.52% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 38 | 0.52% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 39 | 0.52% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 40 | 0.52% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 41 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 42 | 0.52% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 43 | 0.46% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 44 | 0.46% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 45 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 46 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 47 | 0.46% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1071` |
| 48 | 0.46% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 49 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 50 | 0.41% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 51 | 0.41% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 52 | 0.41% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 53 | 0.41% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 54 | 0.41% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 55 | 0.36% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 56 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.36% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 58 | 0.36% | `net.minecraft.world.level.levelgen.Beardifier.compute(DensityFunction$FunctionContext) line: 80` |
| 59 | 0.36% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 60 | 0.36% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 2.63% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 3 | 2.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.48% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 5 | 2.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 6 | 2.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 7 | 2.27% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 8 | 2.17% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 9 | 1.96% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 10 | 1.60% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 11 | 1.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.50% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 13 | 1.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 14 | 1.19% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 15 | 1.14% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 16 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 17 | 0.98% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 19 | 0.88% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 20 | 0.88% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 21 | 0.77% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 22 | 0.77% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 23 | 0.72% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 24 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 25 | 0.67% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 26 | 0.67% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 27 | 0.67% | `net.neoforged.art.internal.EnhancedRemapper.getClass(String) line: 121` |
| 28 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 29 | 0.67% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 30 | 0.62% | `java.lang.String.hashCode() line: 2358` |

GC pauses: 0, total 0 ms


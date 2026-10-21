# JVM hotspots (JFR ExecutionSample)

Samples: 1276

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 2.51% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.43% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 4 | 2.19% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 5 | 2.12% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 7 | 1.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 9 | 1.57% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 10 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 11 | 1.25% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 12 | 1.18% | `java.lang.String.hashCode() line: 2358` |
| 13 | 1.10% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 14 | 1.10% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 15 | 1.10% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 16 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 17 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 18 | 1.02% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 19 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 20 | 0.94% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 21 | 0.94% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 22 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 23 | 0.78% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 24 | 0.71% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 25 | 0.71% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 26 | 0.71% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 27 | 0.71% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 28 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 29 | 0.63% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 31 | 0.63% | `net.minecraft.util.SimpleBitStorage.getAll(IntConsumer) line: 361` |
| 32 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 33 | 0.55% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 36 | 0.55% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 37 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 38 | 0.47% | `java.lang.String.equals(Object) line: 1850` |
| 39 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 40 | 0.47% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 41 | 0.47% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 42 | 0.47% | `java.util.ImmutableCollections$Set12.contains(Object) line: 817` |
| 43 | 0.39% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 44 | 0.39% | `com.sun.crypto.provider.GHASH.blockMult(long[], long[]) line: 93` |
| 45 | 0.39% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 46 | 0.39% | `java.util.HashMap.getNode(Object) line: 587` |
| 47 | 0.39% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 48 | 0.39% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 49 | 0.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 50 | 0.39% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 51 | 0.31% | `java.io.InputStream.readNBytes(int) line: 412` |
| 52 | 0.31% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 53 | 0.31% | `org.objectweb.asm.Type.getReturnTypeOffset(String) line: 380` |
| 54 | 0.31% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 380` |
| 55 | 0.31% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 56 | 0.31% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 57 | 0.31% | `net.minecraft.world.level.levelgen.DensityFunctions$PureTransformer.fillArray(double[], DensityFunction$ContextProvider) line: 855` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$7(int, int, NewChunkHolder, List, int, int) line: 771` |
| 59 | 0.31% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.recalculateNeighbourRequestedPriority() line: 556` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 3.13% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 4 | 2.51% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 5 | 2.43% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 6 | 2.19% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 7 | 2.12% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 8 | 1.80% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 9 | 1.80% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 10 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 11 | 1.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 12 | 1.10% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 13 | 1.10% | `net.minecraft.world.level.levelgen.NoiseChunk.getInterpolatedState() line: 177` |
| 14 | 1.02% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 15 | 0.94% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 16 | 0.94% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 17 | 0.86% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 18 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 926` |
| 19 | 0.78% | `java.lang.String.hashCode() line: 2358` |
| 20 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 21 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.getAll(Consumer) line: 264` |
| 22 | 0.71% | `com.sun.crypto.provider.GHASH.processBlock(byte[], int, long[], long[]) line: 167` |
| 23 | 0.71% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 24 | 0.71% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 25 | 0.71% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 26 | 0.71% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 86` |
| 27 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 28 | 0.63% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, int, int, Object) line: 212` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |

GC pauses: 0, total 0 ms


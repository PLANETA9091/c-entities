# JVM hotspots (JFR ExecutionSample)

Samples: 1111

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.50% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 2 | 4.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 4 | 2.52% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 5 | 2.25% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 6 | 2.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 7 | 2.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 8 | 1.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 10 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 11 | 1.17% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 12 | 1.17% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 13 | 1.08% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 14 | 0.99% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 15 | 0.90% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 16 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 17 | 0.81% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 19 | 0.81% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 20 | 0.72% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 21 | 0.72% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 22 | 0.72% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 23 | 0.72% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 24 | 0.63% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 25 | 0.63% | `java.lang.AbstractStringBuilder.append(String) line: 589` |
| 26 | 0.63% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 27 | 0.63% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 29 | 0.63% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 31 | 0.54% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 32 | 0.54% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 33 | 0.54% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 34 | 0.54% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 35 | 0.54% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 36 | 0.54% | `java.util.ImmutableCollections$Set12.contains(Object) line: 817` |
| 37 | 0.54% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 38 | 0.45% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 39 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 40 | 0.45% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 41 | 0.45% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 42 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.36% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 44 | 0.36% | `com.google.common.base.Suppliers$NonSerializableMemoizingSupplier.get() line: 200` |
| 45 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 47 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 48 | 0.27% | `java.lang.invoke.VarHandleByteArrayAsInts$ArrayHandle.get(VarHandle, Object, int) line: 116` |
| 49 | 0.27% | `java.lang.String.charAt(int) line: 1546` |
| 50 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 51 | 0.27% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 52 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.bsR(int) line: 388` |
| 53 | 0.27% | `java.lang.String.<init>(AbstractStringBuilder, Void) line: 4785` |
| 54 | 0.27% | `org.objectweb.asm.Type.getArgumentCount(String) line: 723` |
| 55 | 0.27% | `java.lang.StringBuilder.<init>() line: 106` |
| 56 | 0.27% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 57 | 0.27% | `java.lang.String.hashCode() line: 2358` |
| 58 | 0.27% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 641` |
| 59 | 0.27% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 4.50% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 3 | 4.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 4 | 2.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.52% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 6 | 2.25% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 7 | 2.07% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 1.53% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 10 | 1.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 11 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 12 | 1.08% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 13 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 14 | 1.08% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.generateFeatures(WorldGenContext, ChunkStep, StaticCache2D, ChunkAccess) line: 145` |
| 16 | 0.99% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 17 | 0.90% | `net.minecraft.world.level.levelgen.NoiseChunk.getInterpolatedState() line: 177` |
| 18 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 19 | 0.81% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 20 | 0.81% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 21 | 0.72% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 22 | 0.72% | `java.util.concurrent.ConcurrentHashMap.addCount(long, int) line: 2354` |
| 23 | 0.72% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 24 | 0.72% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 152` |
| 25 | 0.72% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 26 | 0.63% | `java.lang.StringBuilder.append(String) line: 179` |
| 27 | 0.63% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 28 | 0.63% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 92` |
| 29 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 536` |
| 30 | 0.63% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |

GC pauses: 0, total 0 ms


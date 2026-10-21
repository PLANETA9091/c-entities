# JVM hotspots (JFR ExecutionSample)

Samples: 1233

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 2 | 3.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.68% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 4 | 2.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 5 | 2.11% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1123` |
| 6 | 2.03% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 7 | 1.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 9 | 1.62% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 10 | 1.38% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 11 | 1.30% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 12 | 1.22% | `java.lang.String.hashCode() line: 2358` |
| 13 | 1.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.14% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 15 | 1.05% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 16 | 1.05% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 17 | 1.05% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 18 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 19 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 20 | 0.89% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 21 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 22 | 0.81% | `java.util.HashMap.getNode(Object) line: 578` |
| 23 | 0.81% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 24 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 25 | 0.81% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 26 | 0.73% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 27 | 0.73% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 29 | 0.65% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 30 | 0.57% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 31 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.<init>(int, float) line: 96` |
| 33 | 0.49% | `java.lang.String.equals(Object) line: 1850` |
| 34 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.49% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 127` |
| 37 | 0.49% | `net.minecraft.world.level.chunk.SingleValuePalette.idFor(Object) line: 45` |
| 38 | 0.49% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 39 | 0.49% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 40 | 0.41% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 41 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 42 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 43 | 0.41% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 44 | 0.41% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 45 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 46 | 0.41% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 47 | 0.41% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 48 | 0.41% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 49 | 0.32% | `java.util.HashMap$ValueSpliterator.forEachRemaining(Consumer) line: 1787` |
| 50 | 0.32% | `org.objectweb.asm.ClassReader.<init>(byte[], int, boolean) line: 266` |
| 51 | 0.32% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 52 | 0.32% | `net.neoforged.art.internal.EnhancedRemapper.getClass(String) line: 123` |
| 53 | 0.32% | `net.neoforged.art.internal.EnhancedRemapper$MClass.getMethod(String, String) line: 347` |
| 54 | 0.32% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 55 | 0.32% | `net.minecraft.world.level.levelgen.Beardifier.compute(DensityFunction$FunctionContext) line: 80` |
| 56 | 0.32% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 474` |
| 57 | 0.32% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getAllStarts() line: 195` |
| 58 | 0.32% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.BlockStarLightEngine.checkBlock(LightChunkGetter, int, int, int) line: 94` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.49% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 3 | 3.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 4 | 2.84% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 5 | 2.68% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 6 | 2.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.19% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 2.03% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 9 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 10 | 1.30% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 11 | 1.14% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 12 | 1.05% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 13 | 1.05% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 92` |
| 14 | 1.05% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 15 | 1.05% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 16 | 0.97% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 17 | 0.81% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 18 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 19 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 20 | 0.81% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 21 | 0.73% | `java.lang.AbstractStringBuilder.append(String) line: 592` |
| 22 | 0.73% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 23 | 0.73% | `java.util.HashMap.get(Object) line: 564` |
| 24 | 0.73% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 25 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 27 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 536` |
| 28 | 0.57% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.57% | `java.lang.String.hashCode() line: 2358` |
| 30 | 0.57% | `java.lang.AbstractStringBuilder.append(char) line: 811` |

GC pauses: 0, total 0 ms


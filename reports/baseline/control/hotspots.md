# JVM hotspots (JFR ExecutionSample)

Samples: 1050

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 2.95% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 3 | 2.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 2.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 5 | 2.38% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 6 | 2.38% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 7 | 2.19% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 8 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 9 | 1.43% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 10 | 1.33% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 11 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 12 | 1.14% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 13 | 1.05% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 185` |
| 14 | 1.05% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 16 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 17 | 0.95% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 18 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 19 | 0.86% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 20 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 21 | 0.76% | `java.lang.String.hashCode() line: 2358` |
| 22 | 0.76% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 23 | 0.67% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 24 | 0.67% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 26 | 0.67% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 27 | 0.57% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 28 | 0.57% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 29 | 0.57% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 30 | 0.57% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 31 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 32 | 0.48% | `org.objectweb.asm.commons.Remapper.mapType(String) line: 99` |
| 33 | 0.48% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 34 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 35 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |
| 36 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.<init>(int, float) line: 96` |
| 37 | 0.38% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 38 | 0.38% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 39 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 40 | 0.38% | `java.lang.String.equals(Object) line: 1850` |
| 41 | 0.38% | `java.util.Objects.hashCode(Object) line: 103` |
| 42 | 0.38% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 43 | 0.38% | `java.util.HashMap.computeIfAbsent(Object, Function) line: 1219` |
| 44 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.fillBiomesFromNoise(BiomeResolver, Climate$Sampler, int, int, int) line: 318` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 46 | 0.38% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 279` |
| 47 | 0.38% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForY(double) line: 725` |
| 48 | 0.38% | `it.unimi.dsi.fastutil.ints.IntArraySet.add(int) line: 377` |
| 49 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 50 | 0.29% | `java.math.BigInteger.implMulAdd(int[], int[], int, int, int) line: 3326` |
| 51 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 52 | 0.29% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 54 | 0.29% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 55 | 0.29% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 56 | 0.29% | `org.objectweb.asm.ClassReader.<init>(byte[], int, boolean) line: 266` |
| 57 | 0.29% | `org.objectweb.asm.ByteVector.enlarge(int) line: 365` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.merge(CachedShapeData, CachedShapeData, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, CollisionUtil$MergedVoxelCoordinateList, int) line: 1056` |
| 59 | 0.29% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 647` |
| 60 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 469` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 4.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 2.95% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 4 | 2.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 5 | 2.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 2.38% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 7 | 2.19% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 8 | 2.10% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 9 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 10 | 1.43% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 11 | 1.33% | `net.minecraft.world.level.levelgen.NoiseChunk.getInterpolatedState() line: 177` |
| 12 | 1.24% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 13 | 1.24% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 14 | 1.14% | `java.lang.String.hashCode() line: 2358` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 16 | 1.05% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 17 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 18 | 0.76% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 19 | 0.76% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 86` |
| 20 | 0.67% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 21 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 22 | 0.57% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 23 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 24 | 0.57% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 324` |
| 25 | 0.57% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 26 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 27 | 0.48% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 28 | 0.48% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 29 | 0.48% | `net.neoforged.art.internal.EnhancedRemapper.computeClass(String) line: 147` |
| 30 | 0.48% | `io.papermc.paper.util.StringPool.string(String) line: 32` |

GC pauses: 0, total 0 ms


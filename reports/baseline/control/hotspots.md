# JVM hotspots (JFR ExecutionSample)

Samples: 1213

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 2 | 3.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.47% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 4 | 2.23% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 5 | 2.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 6 | 2.06% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 7 | 1.98% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 8 | 1.57% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 9 | 1.48% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 10 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 11 | 1.40% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 12 | 1.32% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 13 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 14 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 16 | 0.91% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 17 | 0.82% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 18 | 0.82% | `java.lang.String.hashCode() line: 2358` |
| 19 | 0.82% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 20 | 0.82% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 21 | 0.82% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 22 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |
| 23 | 0.74% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 24 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 25 | 0.74% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 26 | 0.74% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 27 | 0.66% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 28 | 0.66% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 29 | 0.66% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 30 | 0.66% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 31 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 32 | 0.58% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 33 | 0.58% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 34 | 0.58% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 35 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.initArrayFromCapacity(int) line: 105` |
| 36 | 0.58% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.initNibble(SWMRNibbleArray, int, int, int, boolean) line: 85` |
| 37 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.49% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 39 | 0.49% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 279` |
| 40 | 0.49% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 41 | 0.49% | `net.minecraft.util.ZeroBitStorage.getAll(IntConsumer) line: 52` |
| 42 | 0.41% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 43 | 0.41% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 44 | 0.41% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 45 | 0.41% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 46 | 0.33% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 47 | 0.33% | `java.util.Arrays.copyOfRange(byte[], int, int) line: 3856` |
| 48 | 0.33% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 49 | 0.33% | `net.neoforged.art.internal.EnhancedRemapper.getClass(String) line: 123` |
| 50 | 0.33% | `net.minecraft.world.phys.shapes.DiscreteVoxelShape.moonrise$getOrCreateCachedShapeData() line: 71` |
| 51 | 0.33% | `net.minecraft.world.level.biome.BiomeManager.getBiome(BlockPos) line: 54` |
| 52 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.getBlockState(int, int, int) line: 77` |
| 53 | 0.33% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 54 | 0.25% | `sun.security.provider.SHA2.implCompress0(byte[], int) line: 193` |
| 55 | 0.25% | `java.lang.AbstractStringBuilder.inflateIfNeededFor(String) line: 1741` |
| 56 | 0.25% | `java.lang.StringLatin1.newString(byte[], int, int) line: 756` |
| 57 | 0.25% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 58 | 0.25% | `java.util.HashMap$ValueSpliterator.forEachRemaining(Consumer) line: 1787` |
| 59 | 0.25% | `java.util.HashMap.getNode(Object) line: 578` |
| 60 | 0.25% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 3 | 3.22% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 4 | 2.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.56% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 6 | 2.23% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 7 | 2.14% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 1.48% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 9 | 1.48% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 10 | 1.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 11 | 1.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 12 | 1.32% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 13 | 1.15% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 14 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 15 | 0.91% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 16 | 0.91% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 17 | 0.91% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 18 | 0.82% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 19 | 0.82% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 20 | 0.82% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.checkUnload() line: 776` |
| 22 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 23 | 0.74% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 25 | 0.74% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 26 | 0.66% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 27 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 28 | 0.66% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 29 | 0.66% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.getAll(Consumer) line: 264` |

GC pauses: 0, total 0 ms


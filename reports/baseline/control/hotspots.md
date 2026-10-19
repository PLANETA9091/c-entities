# JVM hotspots (JFR ExecutionSample)

Samples: 1118

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 2 | 3.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 3.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 2.95% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 5 | 2.24% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 2.15% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 7 | 1.61% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 8 | 1.61% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 9 | 1.61% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 10 | 1.52% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 11 | 1.43% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 12 | 1.43% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 13 | 1.07% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 14 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 15 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 16 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 17 | 0.81% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 18 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 19 | 0.72% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 20 | 0.72% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 21 | 0.72% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 22 | 0.63% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 23 | 0.63% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 24 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 138` |
| 25 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 26 | 0.63% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 27 | 0.54% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 28 | 0.54% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 29 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 30 | 0.54% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 31 | 0.54% | `java.lang.Class.cast(Object) line: 4069` |
| 32 | 0.54% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 33 | 0.54% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$3(int, boolean, Climate$RTree$Node) line: 329` |
| 34 | 0.54% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 35 | 0.54% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 36 | 0.45% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 37 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 38 | 0.45% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 39 | 0.45% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 40 | 0.45% | `java.util.Arrays.fill(int[], int, int, int) line: 3205` |
| 41 | 0.45% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 42 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 43 | 0.36% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 44 | 0.36% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 45 | 0.36% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 46 | 0.36% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 47 | 0.36% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 48 | 0.36% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 49 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 50 | 0.36% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.initArrayFromCapacity(int) line: 105` |
| 52 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |
| 53 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 54 | 0.27% | `java.lang.String.split(char, int, boolean) line: 3358` |
| 55 | 0.27% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 56 | 0.27% | `java.util.HashMap$ValueSpliterator.forEachRemaining(Consumer) line: 1787` |
| 57 | 0.27% | `org.objectweb.asm.Type.getArgumentTypes(String) line: 318` |
| 58 | 0.27% | `java.lang.String.hashCode() line: 2358` |
| 59 | 0.27% | `java.lang.String.equals(Object) line: 1850` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.22% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 3 | 2.77% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 4 | 2.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 6 | 2.24% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.06% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 8 | 1.97% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 9 | 1.61% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 10 | 1.52% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 11 | 1.43% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 12 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 13 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 470` |
| 14 | 1.16% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 15 | 1.07% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 16 | 0.98% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 17 | 0.81% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 18 | 0.81% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 19 | 0.81% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 20 | 0.72% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 21 | 0.72% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 22 | 0.72% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 23 | 0.72% | `net.minecraft.util.ZeroBitStorage.unpack(int[]) line: 58` |
| 24 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 25 | 0.63% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 26 | 0.63% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 27 | 0.63% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 28 | 0.54% | `net.minecraft.world.level.biome.Climate$RTree$SubTree.<init>(List) line: 436` |
| 29 | 0.54% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 30 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.light(LightChunkGetter, ChunkAccess, Boolean[]) line: 902` |

GC pauses: 0, total 0 ms


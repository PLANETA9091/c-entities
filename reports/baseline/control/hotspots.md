# JVM hotspots (JFR ExecutionSample)

Samples: 1321

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.26% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 3 | 2.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 2.42% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 5 | 2.42% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 2.04% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 8 | 1.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 9 | 1.44% | `java.lang.String.hashCode() line: 2358` |
| 10 | 1.29% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 11 | 1.21% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 12 | 1.14% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 14 | 1.06% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 15 | 0.91% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 16 | 0.91% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 17 | 0.83% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 18 | 0.83% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 19 | 0.83% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 20 | 0.83% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 21 | 0.76% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 22 | 0.76% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 23 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 24 | 0.76% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 25 | 0.68% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 26 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 27 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 28 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 29 | 0.61% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 30 | 0.61% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 31 | 0.61% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 32 | 0.61% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 33 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 34 | 0.61% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.61% | `net.minecraft.util.ZeroBitStorage.getAll(IntConsumer) line: 52` |
| 36 | 0.53% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 37 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.53% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 39 | 0.53% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 40 | 0.53% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 185` |
| 41 | 0.53% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 42 | 0.53% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 43 | 0.45% | `java.lang.String.equals(Object) line: 1850` |
| 44 | 0.45% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 45 | 0.45% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 46 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 47 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 48 | 0.38% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 49 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 50 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.38% | `net.minecraft.world.level.LevelWriter.setBlock(BlockPos, BlockState, int) line: 12` |
| 53 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.bsR(int) line: 388` |
| 54 | 0.30% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 55 | 0.30% | `java.util.HashMap$ValueSpliterator.forEachRemaining(Consumer) line: 1787` |
| 56 | 0.30% | `org.objectweb.asm.ByteVector.putUTF8(String) line: 268` |
| 57 | 0.30% | `net.minecraft.world.level.levelgen.NoiseChunk$CacheAllInCell.compute(DensityFunction$FunctionContext) line: 537` |
| 58 | 0.30% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 59 | 0.30% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 60 | 0.30% | `com.google.common.collect.Lists.indexOfRandomAccess(List, Object) line: 1085` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 5.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 3.26% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 4 | 2.50% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 5 | 2.42% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 6 | 2.12% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 7 | 2.04% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 1.74% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 9 | 1.59% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 10 | 1.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 12 | 1.14% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 13 | 0.98% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 14 | 0.98% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 15 | 0.91% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 16 | 0.91% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 17 | 0.83% | `java.lang.String.hashCode() line: 2358` |
| 18 | 0.83% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 19 | 0.83% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 20 | 0.83% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 21 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 22 | 0.76% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 23 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.getAll(Consumer) line: 264` |
| 26 | 0.61% | `java.util.concurrent.ConcurrentHashMap.addCount(long, int) line: 2354` |
| 27 | 0.61% | `org.objectweb.asm.commons.Remapper.mapType(Type) line: 78` |
| 28 | 0.61% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 29 | 0.53% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 30 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |

GC pauses: 0, total 0 ms


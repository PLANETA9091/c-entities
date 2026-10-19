# JVM hotspots (JFR ExecutionSample)

Samples: 886

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.72% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.93% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 4 | 2.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 1.92% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 1.92% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 375` |
| 7 | 1.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 8 | 1.69% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 9 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 10 | 1.47% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 11 | 1.47% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 12 | 1.47% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 13 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 14 | 1.24% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 15 | 1.24% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 16 | 1.24% | `net.minecraft.world.level.chunk.LevelChunkSection.getBlockState(int, int, int) line: 77` |
| 17 | 1.13% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 18 | 1.13% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 19 | 1.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 20 | 1.02% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 21 | 0.90% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 22 | 0.90% | `java.lang.String.hashCode() line: 2358` |
| 23 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 24 | 0.79% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 25 | 0.79% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 26 | 0.79% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 27 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 28 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 29 | 0.68% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 30 | 0.68% | `java.lang.String.equals(Object) line: 1850` |
| 31 | 0.68% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 32 | 0.68% | `net.minecraft.world.level.levelgen.synth.PerlinNoise.wrap(double) line: 190` |
| 33 | 0.56% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 35 | 0.45% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 36 | 0.45% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 37 | 0.45% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 38 | 0.45% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 39 | 0.45% | `net.minecraft.world.level.chunk.HashMapPalette.<init>(IdMap, int, PaletteResize) line: 30` |
| 40 | 0.34% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 41 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 42 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 43 | 0.34% | `java.util.regex.Matcher.quoteReplacement(String) line: 854` |
| 44 | 0.34% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 45 | 0.34% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3721` |
| 46 | 0.34% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 47 | 0.34% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 48 | 0.34% | `net.minecraft.world.level.levelgen.SurfaceRules$Context.lambda$updateY$0(int, int, int) line: 317` |
| 49 | 0.34% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 50 | 0.34% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 152` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 52 | 0.34% | `com.google.common.base.Suppliers$NonSerializableMemoizingSupplier.get() line: 200` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.BlockStarLightEngine.checkBlock(LightChunkGetter, int, int, int) line: 94` |
| 54 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 55 | 0.34% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 56 | 0.34% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |
| 57 | 0.23% | `sun.security.util.math.intpoly.IntegerPolynomialP256.mult(long[], long[], long[]) line: 323` |
| 58 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 59 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 60 | 0.23% | `net.neoforged.srgutils.MappingFile.lambda$write$6(IMappingFile$INode, IMappingFile$INode) line: 115` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 2 | 6.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 3.84% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 4 | 3.72% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 5 | 2.93% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.92% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 1.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 9 | 1.47% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 10 | 1.47% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 11 | 1.47% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 12 | 1.47% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 13 | 1.47% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 14 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 15 | 1.35% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 16 | 1.24% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 17 | 1.02% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 19 | 0.90% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.getBlockState(int, int, int) line: 287` |
| 20 | 0.79% | `java.util.regex.Matcher.appendReplacement(StringBuffer, String) line: 951` |
| 21 | 0.79% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 370` |
| 22 | 0.68% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 23 | 0.68% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 24 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 25 | 0.56% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 26 | 0.56% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 92` |
| 27 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, int, int, Object) line: 212` |
| 28 | 0.45% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 29 | 0.45% | `net.neoforged.srgutils.MappingFile.remapDescriptor(String) line: 107` |
| 30 | 0.45% | `java.util.concurrent.ConcurrentHashMap.addCount(long, int) line: 2354` |

GC pauses: 0, total 0 ms


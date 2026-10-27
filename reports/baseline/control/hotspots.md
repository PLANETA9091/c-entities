# JVM hotspots (JFR ExecutionSample)

Samples: 1061

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 3 | 3.49% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 4 | 3.11% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 2.07% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 6 | 1.79% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 7 | 1.70% | `net.minecraft.world.level.chunk.SingleValuePalette.idFor(Object) line: 45` |
| 8 | 1.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 9 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 10 | 1.32% | `java.lang.String.hashCode() line: 2358` |
| 11 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 12 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 1.13% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 14 | 1.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 15 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 16 | 0.94% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 17 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 18 | 0.94% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 19 | 0.85% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 20 | 0.75% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 21 | 0.75% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 22 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 23 | 0.66% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 24 | 0.66% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 25 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 375` |
| 26 | 0.57% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 27 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.57% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 29 | 0.57% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 30 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 31 | 0.57% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 32 | 0.57% | `net.minecraft.world.level.levelgen.Beardifier.compute(DensityFunction$FunctionContext) line: 80` |
| 33 | 0.57% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 34 | 0.57% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 35 | 0.47% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 36 | 0.47% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 37 | 0.47% | `net.minecraft.world.level.levelgen.SurfaceRules$BiomeConditionSource$1BiomeCondition.compute() line: 188` |
| 38 | 0.47% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 39 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 40 | 0.38% | `com.google.common.base.Suppliers$NonSerializableMemoizingSupplier.get() line: 200` |
| 41 | 0.38% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 42 | 0.38% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 274` |
| 43 | 0.38% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 44 | 0.38% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 45 | 0.38% | `net.minecraft.server.level.ChunkMap.isChunkNearPlayer(ChunkMap, ChunkPos, LevelChunk) line: 706` |
| 46 | 0.38% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 47 | 0.28% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 48 | 0.28% | `java.lang.invoke.VarHandleByteArrayAsLongs$ArrayHandle.index(byte[], int) line: 101` |
| 49 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 50 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.28% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 52 | 0.28% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3721` |
| 53 | 0.28% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 54 | 0.28% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 55 | 0.28% | `java.lang.String.equals(Object) line: 1850` |
| 56 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 57 | 0.28% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$3(int, boolean, Climate$RTree$Node) line: 329` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 60 | 0.28% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 548` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 4.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 3 | 3.49% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 4 | 3.11% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 5 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 6 | 2.07% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 7 | 1.70% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 8 | 1.70% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 9 | 1.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 10 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 11 | 1.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 12 | 1.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 13 | 1.23% | `java.util.regex.Matcher.appendReplacement(StringBuffer, String) line: 951` |
| 14 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.13% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 16 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 17 | 1.04% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 18 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 19 | 0.94% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 20 | 0.85% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 21 | 0.75% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 22 | 0.75% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 473` |
| 23 | 0.75% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 184` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 25 | 0.66% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 279` |
| 26 | 0.66% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 27 | 0.66% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 28 | 0.57% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 29 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 940` |
| 30 | 0.57% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |

GC pauses: 0, total 0 ms


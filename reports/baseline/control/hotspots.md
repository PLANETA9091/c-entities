# JVM hotspots (JFR ExecutionSample)

Samples: 616

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 2 | 4.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 3.57% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 4 | 2.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 5 | 2.27% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 6 | 1.62% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 7 | 1.46% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 8 | 1.30% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 9 | 1.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 10 | 0.97% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 11 | 0.97% | `java.lang.String.hashCode() line: 2358` |
| 12 | 0.97% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 13 | 0.97% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 14 | 0.97% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 15 | 0.97% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 16 | 0.97% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 17 | 0.97% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 19 | 0.81% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 20 | 0.81% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 21 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 22 | 0.81% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 23 | 0.81% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 24 | 0.65% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 25 | 0.65% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 647` |
| 26 | 0.65% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 27 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |
| 28 | 0.49% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 29 | 0.49% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 30 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 31 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.49% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 33 | 0.49% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 34 | 0.49% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 37 | 0.49% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 38 | 0.49% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 548` |
| 39 | 0.49% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 40 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 41 | 0.49% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 42 | 0.49% | `java.util.ImmutableCollections$Set12.contains(Object) line: 817` |
| 43 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 44 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.bsGetBit() line: 410` |
| 45 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 46 | 0.32% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 47 | 0.32% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 48 | 0.32% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 49 | 0.32% | `java.lang.String.equals(Object) line: 1850` |
| 50 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 51 | 0.32% | `java.util.BitSet.expandTo(int) line: 355` |
| 52 | 0.32% | `java.math.BigInteger.montReduce(int[], int[], int, int) line: 3249` |
| 53 | 0.32% | `java.util.HashMap$HashIterator.nextNode() line: 1609` |
| 54 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 751` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 64` |
| 56 | 0.32% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 279` |
| 57 | 0.32% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 29` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$5(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1651` |
| 59 | 0.32% | `net.minecraft.world.level.levelgen.NoiseChunk$CacheAllInCell.compute(DensityFunction$FunctionContext) line: 537` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.checkChunkEdge(LightChunkGetter, ChunkAccess, int, int, int) line: 531` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 2 | 4.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 4.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 4 | 3.25% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 5 | 2.27% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 6 | 1.62% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 7 | 1.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 8 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 9 | 1.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 536` |
| 10 | 1.14% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 11 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 0.97% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 184` |
| 13 | 0.97% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 14 | 0.97% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 15 | 0.97% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 16 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 18 | 0.97% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 19 | 0.97% | `net.minecraft.nbt.ListTag.write(DataOutput) line: 194` |
| 20 | 0.81% | `net.minecraft.world.level.biome.Climate$RTree$SubTree.<init>(List) line: 436` |
| 21 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 22 | 0.81% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 152` |
| 24 | 0.81% | `net.minecraft.world.level.levelgen.NoiseChunk.getInterpolatedState() line: 177` |
| 25 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 26 | 0.65% | `java.util.HashMap.hash(Object) line: 338` |
| 27 | 0.65% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isJoinNonEmpty(VoxelShape, VoxelShape, BooleanOp) line: 1336` |
| 28 | 0.65% | `java.math.MutableBigInteger.divideMagnitude(MutableBigInteger, MutableBigInteger, boolean) line: 1592` |
| 29 | 0.65% | `java.math.BigInteger.implMontgomerySquare(int[], int[], int, long, int[]) line: 2996` |
| 30 | 0.65% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |

GC pauses: 0, total 0 ms


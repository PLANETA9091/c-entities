# JVM hotspots (JFR ExecutionSample)

Samples: 770

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.77% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 3.12% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 4 | 2.99% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 2.47% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 6 | 2.21% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 7 | 1.69% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 8 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 9 | 1.30% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 10 | 1.30% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 11 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 12 | 1.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 13 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.17% | `java.lang.String.hashCode() line: 2358` |
| 15 | 1.17% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 16 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 17 | 1.04% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 18 | 1.04% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 19 | 0.91% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 20 | 0.91% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 21 | 0.91% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 22 | 0.78% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 23 | 0.78% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 24 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 25 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.65% | `java.lang.String.equals(Object) line: 1850` |
| 27 | 0.65% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 28 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 29 | 0.65% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 31 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 32 | 0.65% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 33 | 0.52% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 34 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 35 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 36 | 0.52% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 37 | 0.52% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 38 | 0.52% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 39 | 0.52% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 40 | 0.52% | `net.minecraft.world.level.levelgen.DensityFunctions$PureTransformer.fillArray(double[], DensityFunction$ContextProvider) line: 855` |
| 41 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 42 | 0.52% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 43 | 0.52% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 44 | 0.39% | `java.io.ByteArrayInputStream.read() line: 147` |
| 45 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 46 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 47 | 0.39% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 48 | 0.39% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 49 | 0.39% | `java.util.BitSet.expandTo(int) line: 355` |
| 50 | 0.39% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 51 | 0.39% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$3(int, boolean, Climate$RTree$Node) line: 329` |
| 52 | 0.39% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1817` |
| 53 | 0.39% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 54 | 0.39% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 474` |
| 55 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 56 | 0.39% | `com.google.common.base.Suppliers$NonSerializableMemoizingSupplier.get() line: 200` |
| 57 | 0.39% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 58 | 0.39% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 59 | 0.26% | `com.sun.crypto.provider.AESCrypt.implEncryptBlock(byte[], int, byte[], int) line: 1128` |
| 60 | 0.26% | `com.sun.crypto.provider.GHASH.blockMult(long[], long[]) line: 93` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 2 | 3.77% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 3 | 3.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 3.51% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 5 | 2.99% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 6 | 2.86% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 7 | 2.86% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 8 | 2.47% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 9 | 2.21% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 10 | 1.69% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 11 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 12 | 1.43% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 13 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 15 | 1.30% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 16 | 1.30% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 17 | 1.17% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.generateFeatures(WorldGenContext, ChunkStep, StaticCache2D, ChunkAccess) line: 145` |
| 19 | 1.04% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 20 | 1.04% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 86` |
| 21 | 1.04% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 22 | 0.91% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 23 | 0.91% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 24 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 25 | 0.65% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 26 | 0.65% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 27 | 0.65% | `io.papermc.paper.util.StringPool.string(String) line: 32` |
| 28 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 29 | 0.65% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 152` |

GC pauses: 0, total 0 ms


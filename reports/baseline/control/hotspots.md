# JVM hotspots (JFR ExecutionSample)

Samples: 1361

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.31% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.57% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 4 | 2.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 5 | 2.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.98% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 7 | 1.98% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 8 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 9 | 1.47% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 10 | 1.47% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 11 | 1.40% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 12 | 1.40% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 13 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 14 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 1.25% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 16 | 0.96% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 17 | 0.88% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 18 | 0.88% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 19 | 0.88% | `java.lang.String.hashCode() line: 2358` |
| 20 | 0.81% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 21 | 0.73% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 22 | 0.73% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 23 | 0.73% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 24 | 0.66% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 25 | 0.66% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 26 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 27 | 0.59% | `java.lang.String.equals(Object) line: 1850` |
| 28 | 0.59% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 30 | 0.59% | `net.minecraft.world.level.levelgen.WorldgenRandom.next(int) line: 34` |
| 31 | 0.59% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 32 | 0.59% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 33 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 34 | 0.51% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 36 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 228` |
| 37 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 38 | 0.44% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$3(int, boolean, Climate$RTree$Node) line: 329` |
| 39 | 0.44% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 40 | 0.44% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 279` |
| 41 | 0.44% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 474` |
| 42 | 0.44% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 43 | 0.37% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 44 | 0.37% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 45 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.fillBiomesFromNoise(BiomeResolver, Climate$Sampler, int, int, int) line: 318` |
| 46 | 0.37% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 47 | 0.37% | `net.minecraft.world.level.levelgen.DensityFunction$SimpleFunction.fillArray(double[], DensityFunction$ContextProvider) line: 100` |
| 48 | 0.37% | `java.lang.String.length() line: 1508` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.<init>(int, float) line: 96` |
| 50 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 51 | 0.29% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 52 | 0.29% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 53 | 0.29% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 54 | 0.29% | `org.objectweb.asm.ClassReader.readUnsignedShort(int) line: 3623` |
| 55 | 0.29% | `net.neoforged.art.internal.EnhancedRemapper.getClass(String) line: 123` |
| 56 | 0.29% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2056` |
| 57 | 0.29% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 58 | 0.29% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 59 | 0.29% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 548` |
| 60 | 0.29% | `java.util.Map.computeIfAbsent(Object, Function) line: 1066` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.31% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 3 | 3.23% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 4 | 2.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 5 | 2.57% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 6 | 2.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 8 | 1.84% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 9 | 1.54% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 10 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 11 | 1.47% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 12 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 13 | 0.96% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 14 | 0.88% | `java.lang.String.hashCode() line: 2358` |
| 15 | 0.81% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 16 | 0.81% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 17 | 0.73% | `org.objectweb.asm.commons.Remapper.mapType(Type) line: 78` |
| 18 | 0.73% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 19 | 0.73% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 20 | 0.73% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 21 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 22 | 0.66% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.generateFeatures(WorldGenContext, ChunkStep, StaticCache2D, ChunkAccess) line: 145` |
| 23 | 0.59% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 24 | 0.59% | `org.objectweb.asm.SymbolTable.addConstantNameAndType(String, String) line: 766` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |
| 26 | 0.59% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 27 | 0.59% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 26` |
| 28 | 0.59% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.59% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |

GC pauses: 0, total 0 ms


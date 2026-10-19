# JVM hotspots (JFR ExecutionSample)

Samples: 1114

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 2.78% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 2.69% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 4 | 2.69% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 2.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.97% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 7 | 1.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 8 | 1.44% | `java.lang.String.hashCode() line: 2358` |
| 9 | 1.44% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 10 | 1.35% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 11 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 12 | 1.17% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 13 | 1.08% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 14 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 16 | 0.90% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 17 | 0.90% | `net.minecraft.world.level.biome.BiomeManager.getBiome(BlockPos) line: 54` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 19 | 0.81% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 20 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 21 | 0.72% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 22 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 23 | 0.72% | `java.lang.String.equals(Object) line: 1850` |
| 24 | 0.72% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 26 | 0.63% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 27 | 0.63% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 28 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 29 | 0.63% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 30 | 0.63% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 31 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 32 | 0.63% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 33 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |
| 34 | 0.54% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 35 | 0.54% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.getBlockState(int, int, int) line: 77` |
| 37 | 0.45% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 38 | 0.45% | `java.util.HashMap.getNode(Object) line: 587` |
| 39 | 0.45% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 40 | 0.45% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 41 | 0.45% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 42 | 0.45% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 43 | 0.45% | `net.minecraft.world.level.levelgen.Beardifier.compute(DensityFunction$FunctionContext) line: 80` |
| 44 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 45 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 46 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.calculateLightValue(LightChunkGetter, int, int, int, int) line: 327` |
| 47 | 0.36% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.36% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 50 | 0.36% | `com.google.gson.stream.JsonReader.nextQuotedValue(char) line: 1107` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 52 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 53 | 0.36% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 540` |
| 54 | 0.36% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 55 | 0.36% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 56 | 0.36% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 57 | 0.36% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 58 | 0.27% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 59 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 60 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 3.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 2.78% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 4 | 2.78% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 5 | 2.69% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 6 | 1.97% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 7 | 1.80% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 8 | 1.71% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 9 | 1.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 11 | 1.35% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 12 | 1.35% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 13 | 1.26% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 14 | 1.17% | `net.minecraft.world.level.levelgen.SurfaceRules$Context.lambda$updateY$0(int, int, int) line: 317` |
| 15 | 0.90% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 16 | 0.90% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 17 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 18 | 0.81% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 19 | 0.81% | `java.lang.String.hashCode() line: 2358` |
| 20 | 0.72% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 21 | 0.63% | `java.math.BigInteger.multiplyToLen(int[], int, int[], int, int[]) line: 1797` |
| 22 | 0.63% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 23 | 0.63% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 24 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |
| 25 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 470` |
| 26 | 0.63% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 27 | 0.63% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 184` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.checkUnload() line: 776` |
| 29 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 885` |
| 30 | 0.54% | `java.util.HashMap.get(Object) line: 564` |

GC pauses: 0, total 0 ms


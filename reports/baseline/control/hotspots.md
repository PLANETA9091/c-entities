# JVM hotspots (JFR ExecutionSample)

Samples: 1076

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 3 | 2.60% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 4 | 2.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 5 | 2.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 2.04% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 7 | 1.77% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 8 | 1.77% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 9 | 1.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 1.49% | `java.lang.String.hashCode() line: 2358` |
| 11 | 1.39% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 12 | 1.30% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 13 | 1.30% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 14 | 1.30% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 15 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 16 | 1.12% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 17 | 1.12% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 18 | 1.02% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 19 | 1.02% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 20 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 21 | 0.74% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 22 | 0.74% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 23 | 0.65% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 24 | 0.65% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 25 | 0.65% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 27 | 0.56% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 28 | 0.56% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 29 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 31 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState) line: 93` |
| 32 | 0.56% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 34 | 0.46% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 35 | 0.46% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 36 | 0.46% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 37 | 0.46% | `net.minecraft.world.level.levelgen.synth.PerlinNoise.wrap(double) line: 190` |
| 38 | 0.46% | `java.util.ImmutableCollections$Set12.contains(Object) line: 817` |
| 39 | 0.46% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 40 | 0.46% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 41 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 42 | 0.37% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 43 | 0.37% | `java.lang.invoke.VarHandleByteArrayAsInts$ArrayHandle.get(VarHandle, Object, int) line: 116` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 45 | 0.37% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 46 | 0.37% | `java.lang.String.equals(Object) line: 1850` |
| 47 | 0.37% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 48 | 0.37% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |
| 49 | 0.37% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 50 | 0.37% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$3(int, boolean, Climate$RTree$Node) line: 329` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 52 | 0.37% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 53 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 54 | 0.37% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 55 | 0.37% | `java.util.Objects.equals(Object, Object) line: 64` |
| 56 | 0.37% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 57 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 58 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 59 | 0.28% | `java.lang.invoke.VarHandleByteArrayAsLongs$ArrayHandle.index(byte[], int) line: 101` |
| 60 | 0.28% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 2 | 4.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 3.72% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 4 | 3.53% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 5 | 2.60% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 6 | 2.04% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 7 | 1.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.77% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 9 | 1.58% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 10 | 1.39% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 11 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 1.30% | `net.minecraft.world.level.levelgen.NoiseChunk.getInterpolatedState() line: 177` |
| 13 | 1.02% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 14 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 470` |
| 15 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 16 | 0.65% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |
| 17 | 0.65% | `org.objectweb.asm.SymbolTable.hash(int, String) line: 1378` |
| 18 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 19 | 0.65% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 370` |
| 20 | 0.65% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.compute(DensityFunction$FunctionContext) line: 570` |
| 21 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 22 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 23 | 0.65% | `net.minecraft.nbt.ListTag.write(DataOutput) line: 194` |
| 24 | 0.56% | `java.util.concurrent.ConcurrentHashMap.addCount(long, int) line: 2354` |
| 25 | 0.56% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 26 | 0.56% | `java.lang.AbstractStringBuilder.append(String) line: 592` |
| 27 | 0.56% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 28 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 29 | 0.56% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |

GC pauses: 0, total 0 ms


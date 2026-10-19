# JVM hotspots (JFR ExecutionSample)

Samples: 749

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 3.87% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 3 | 3.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 4 | 3.60% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 5 | 1.87% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 1.60% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 7 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 8 | 1.34% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 9 | 1.34% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 10 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 11 | 1.07% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 12 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 13 | 1.07% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 14 | 1.07% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 15 | 1.07% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 16 | 1.07% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 17 | 0.93% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 18 | 0.93% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 19 | 0.93% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 20 | 0.93% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 21 | 0.93% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 22 | 0.80% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 23 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 24 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 25 | 0.80% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 26 | 0.80% | `java.lang.String.hashCode() line: 2358` |
| 27 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 28 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$BiomeConditionSource$1BiomeCondition.compute() line: 188` |
| 29 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 30 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 31 | 0.67% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 32 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 33 | 0.53% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 34 | 0.53% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 35 | 0.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 36 | 0.53% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 37 | 0.53% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 474` |
| 38 | 0.53% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 39 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 40 | 0.53% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 41 | 0.40% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 42 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 43 | 0.40% | `java.lang.StringBuilder.<init>() line: 106` |
| 44 | 0.40% | `java.lang.StringLatin1.newString(byte[], int, int) line: 756` |
| 45 | 0.40% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 46 | 0.40% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 47 | 0.40% | `java.lang.String.equals(Object) line: 1850` |
| 48 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1734` |
| 49 | 0.40% | `org.objectweb.asm.commons.MethodRemapper.visitMethodInsn(int, String, String, String, boolean) line: 168` |
| 50 | 0.40% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.ints.IntArraySet.add(int) line: 377` |
| 52 | 0.40% | `net.minecraft.world.level.levelgen.LegacyRandomSource.next(int) line: 44` |
| 53 | 0.40% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 54 | 0.40% | `net.minecraft.world.level.levelgen.Beardifier.compute(DensityFunction$FunctionContext) line: 80` |
| 55 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 56 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 57 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 375` |
| 58 | 0.27% | `com.sun.crypto.provider.GHASH.blockMult(long[], long[]) line: 67` |
| 59 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 60 | 0.27% | `java.io.ByteArrayInputStream.read() line: 147` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 2 | 6.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 3.87% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 4 | 3.60% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 5 | 2.80% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 6 | 2.00% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 7 | 1.74% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 8 | 1.60% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 9 | 1.60% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 10 | 1.34% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 11 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 926` |
| 12 | 1.07% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 370` |
| 13 | 1.07% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 181` |
| 14 | 0.93% | `java.lang.String.hashCode() line: 2358` |
| 15 | 0.93% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 16 | 0.93% | `java.util.Collections$UnmodifiableMap.values() line: 1690` |
| 17 | 0.80% | `java.util.regex.Matcher.appendReplacement(StringBuffer, String) line: 951` |
| 18 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 19 | 0.80% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 20 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 21 | 0.67% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 22 | 0.67% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 23 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 24 | 0.67% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, WorldGenerationContext, RandomState, StructureManager, BiomeManager, Registry, Blender) line: 247` |
| 25 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 473` |
| 26 | 0.67% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.sampleAndLerp(int, int, int, double, double, double, double) line: 92` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 28 | 0.67% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 536` |
| 29 | 0.67% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 30 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

GC pauses: 0, total 0 ms


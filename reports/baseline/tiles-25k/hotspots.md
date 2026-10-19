# JVM hotspots (JFR ExecutionSample)

Samples: 2174

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.61% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 3 | 4.65% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.85% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.71% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.12% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 7 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.70% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.52% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 10 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.43% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.20% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 15 | 1.15% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 16 | 1.06% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 17 | 1.01% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 18 | 1.01% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 19 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 20 | 0.87% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 22 | 0.83% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.78% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 25 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.69% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 27 | 0.64% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 28 | 0.64% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 29 | 0.60% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 30 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.55% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 32 | 0.55% | `net.minecraft.world.level.block.entity.BlockEntity.getBlockPos() line: 244` |
| 33 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 34 | 0.51% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 35 | 0.46% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 36 | 0.46% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 37 | 0.46% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 38 | 0.46% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 39 | 0.41% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 40 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 41 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 43 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 228` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 45 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.37% | `java.lang.String.hashCode() line: 2358` |
| 47 | 0.37% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 48 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 49 | 0.37% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 50 | 0.32% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 51 | 0.32% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 52 | 0.32% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 53 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 54 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 55 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 56 | 0.28% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3721` |
| 57 | 0.28% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 58 | 0.28% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 59 | 0.28% | `org.objectweb.asm.ByteVector.putUTF8(String) line: 268` |
| 60 | 0.28% | `java.util.Objects.equals(Object, Object) line: 64` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 2 | 4.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 3 | 3.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 3.36% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 5 | 2.99% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 6 | 2.81% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 7 | 2.25% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.12% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 9 | 1.61% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 10 | 1.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 11 | 1.43% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 12 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 13 | 1.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 14 | 1.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 15 | 1.15% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 16 | 1.15% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 17 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.10% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 19 | 1.10% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 20 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 21 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 22 | 1.01% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 23 | 0.97% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 24 | 0.83% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.64% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 26 | 0.60% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 27 | 0.60% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 184` |
| 28 | 0.60% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 29 | 0.60% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 30 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |

GC pauses: 0, total 0 ms


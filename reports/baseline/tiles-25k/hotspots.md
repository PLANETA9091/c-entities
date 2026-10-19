# JVM hotspots (JFR ExecutionSample)

Samples: 2080

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.58% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.56% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.60% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.12% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 1.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.78% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.73% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 9 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.49% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 12 | 1.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 13 | 1.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 1.11% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 16 | 1.11% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 17 | 0.96% | `java.lang.String.hashCode() line: 2358` |
| 18 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 19 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 20 | 0.82% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 21 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 23 | 0.72% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 24 | 0.72% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 25 | 0.67% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 26 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 27 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 28 | 0.62% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.62% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 31 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 32 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.53% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 34 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 35 | 0.48% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 36 | 0.48% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 37 | 0.48% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 39 | 0.48% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 40 | 0.43% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 41 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 42 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 43 | 0.43% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 44 | 0.43% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 45 | 0.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 46 | 0.38% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 47 | 0.38% | `java.lang.String.equals(Object) line: 1850` |
| 48 | 0.38% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 49 | 0.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 50 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 51 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 52 | 0.34% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 53 | 0.34% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 54 | 0.34% | `net.minecraft.world.level.chunk.SingleValuePalette.idFor(Object) line: 45` |
| 55 | 0.34% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 56 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 57 | 0.34% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 58 | 0.29% | `java.util.HashMap$ValueSpliterator.forEachRemaining(Consumer) line: 1787` |
| 59 | 0.29% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 60 | 0.29% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 3.56% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 3 | 3.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.74% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 5 | 2.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 6 | 2.16% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 7 | 1.92% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 8 | 1.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 9 | 1.78% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 1.78% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 11 | 1.73% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 12 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.read0() line: 228` |
| 13 | 1.39% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 14 | 1.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.15% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 16 | 1.11% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 17 | 1.11% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 18 | 1.06% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 19 | 1.01% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 20 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 21 | 0.96% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 22 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.91% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 24 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 25 | 0.62% | `org.objectweb.asm.ClassReader.readUtf(int, char[]) line: 3702` |
| 26 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 27 | 0.62% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 28 | 0.58% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 936` |
| 29 | 0.58% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(BlockPos) line: 108` |
| 30 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 2171

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 5.20% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 5.16% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.96% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 6 | 2.26% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 7 | 2.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.93% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 9 | 1.84% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.34% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 13 | 1.20% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.20% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 15 | 1.20% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 16 | 1.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 17 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 18 | 1.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 20 | 1.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.97% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.78% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 25 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 26 | 0.74% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 27 | 0.74% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 28 | 0.74% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 29 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 30 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 32 | 0.55% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 33 | 0.55% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 34 | 0.51% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 35 | 0.51% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 36 | 0.46% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 37 | 0.46% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 39 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 40 | 0.41% | `java.lang.String.hashCode() line: 2358` |
| 41 | 0.41% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 185` |
| 43 | 0.41% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 44 | 0.41% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 45 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 48 | 0.37% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 49 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 50 | 0.32% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 51 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 52 | 0.32% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 53 | 0.32% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 54 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 55 | 0.28% | `java.util.HashMap.computeIfAbsent(Object, Function) line: 1219` |
| 56 | 0.28% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 57 | 0.28% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 58 | 0.28% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 59 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 60 | 0.28% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.39% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.10% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 3 | 3.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.40% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 5 | 2.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 6 | 2.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 7 | 1.93% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 8 | 1.80% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.80% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 10 | 1.75% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 11 | 1.57% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 12 | 1.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 15 | 1.34% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 16 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 17 | 1.24% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 18 | 1.20% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 19 | 1.20% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 20 | 1.20% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 21 | 1.20% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 22 | 1.15% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 23 | 1.11% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 24 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 25 | 0.74% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 26 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 27 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 28 | 0.74% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 29 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 30 | 0.69% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |

GC pauses: 0, total 0 ms


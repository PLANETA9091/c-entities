# JVM hotspots (JFR ExecutionSample)

Samples: 3387

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 9.03% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 8.77% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 4 | 4.43% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 4.04% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 2.95% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 7 | 2.63% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.51% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 9 | 2.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 2.13% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.98% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 12 | 1.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 14 | 1.62% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 15 | 1.54% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 16 | 1.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 17 | 1.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 1.00% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 21 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.59% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 23 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 26 | 0.41% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 27 | 0.41% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 28 | 0.38% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 29 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.35% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 31 | 0.32% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 32 | 0.32% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 33 | 0.30% | `java.lang.String.hashCode() line: 2358` |
| 34 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 35 | 0.27% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 36 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 37 | 0.27% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 38 | 0.27% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 39 | 0.27% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 40 | 0.27% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 41 | 0.27% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 42 | 0.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 43 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 44 | 0.24% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 45 | 0.24% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 46 | 0.24% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 63` |
| 47 | 0.24% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 48 | 0.24% | `net.minecraft.world.level.block.entity.BlockEntity.getLevel() line: 90` |
| 49 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 50 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 51 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 52 | 0.21% | `java.lang.String.equals(Object) line: 1850` |
| 53 | 0.21% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 54 | 0.21% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 55 | 0.21% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 56 | 0.21% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 57 | 0.21% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 58 | 0.21% | `net.minecraft.world.level.LevelHeightAccessor.getSectionIndexFromSectionY(int) line: 44` |
| 59 | 0.21% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 60 | 0.21% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 6.76% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 4.16% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 2.95% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 5 | 2.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 6 | 2.63% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 7 | 2.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 8 | 2.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.13% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 10 | 2.10% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 11 | 2.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.92% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 13 | 1.68% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 14 | 1.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 15 | 1.54% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 16 | 1.54% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 17 | 1.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 20 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 1.00% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 0.94% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 23 | 0.86% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 24 | 0.80% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 25 | 0.80% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 26 | 0.71% | `net.minecraft.world.level.Level.getBlockState(BlockPos) line: 1340` |
| 27 | 0.53% | `net.minecraft.world.level.ChunkPos.asLong(BlockPos) line: 89` |
| 28 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.44% | `net.minecraft.world.level.block.AbstractFurnaceBlock.lambda$createFurnaceTicker$0(ServerLevel, Level, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 90` |
| 30 | 0.41% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |

GC pauses: 0, total 0 ms


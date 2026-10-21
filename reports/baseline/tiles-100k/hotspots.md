# JVM hotspots (JFR ExecutionSample)

Samples: 3872

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 7.15% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 6.74% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.85% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 3.77% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.87% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.74% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.71% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 9 | 2.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 2.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 2.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 13 | 1.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.58% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 15 | 1.52% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 16 | 1.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 17 | 1.34% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 18 | 1.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 19 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 20 | 1.06% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 21 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 1.01% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 23 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1312` |
| 25 | 0.54% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 26 | 0.49% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 27 | 0.49% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 28 | 0.49% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 29 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 30 | 0.44% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 31 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 32 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 33 | 0.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.41% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 35 | 0.41% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 36 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 37 | 0.36% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 38 | 0.34% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 39 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 40 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 41 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 42 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 43 | 0.31% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 44 | 0.28% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 45 | 0.28% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 46 | 0.28% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 47 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 48 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.26% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 50 | 0.26% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 51 | 0.26% | `net.minecraft.world.level.block.entity.BlockEntityType.getBlockEntity(BlockGetter, BlockPos) line: 277` |
| 52 | 0.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 53 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 54 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 55 | 0.23% | `java.lang.String.hashCode() line: 2358` |
| 56 | 0.23% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.isLit() line: 152` |
| 57 | 0.21% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 58 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 60 | 0.21% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 6.51% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 4.24% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 2.84% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 2.79% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 6 | 2.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 7 | 2.53% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 8 | 2.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 9 | 2.22% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 10 | 2.17% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.70% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 12 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.58% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 14 | 1.52% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 15 | 1.52% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 16 | 1.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.39% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 18 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.32% | `net.minecraft.world.level.Level.getBlockState(BlockPos) line: 1340` |
| 20 | 1.21% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 21 | 1.06% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 22 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 23 | 1.01% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 24 | 0.98% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 25 | 0.85% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 26 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 27 | 0.75% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 28 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 29 | 0.49% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.49% | `net.minecraft.world.level.block.state.StateHolder.getValue(Property) line: 103` |

GC pauses: 0, total 0 ms


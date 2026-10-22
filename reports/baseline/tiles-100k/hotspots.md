# JVM hotspots (JFR ExecutionSample)

Samples: 2733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.03% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 8.67% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 6.33% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 5.38% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 5.31% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 6 | 5.09% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 3.59% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 3.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 10 | 1.72% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 12 | 1.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 13 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.21% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 16 | 1.17% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 18 | 1.02% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 19 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 21 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.73% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 23 | 0.70% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 24 | 0.70% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 25 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 27 | 0.48% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 28 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 29 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 31 | 0.40% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 32 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 33 | 0.37% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 34 | 0.33% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 35 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 36 | 0.33% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 37 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 38 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 39 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.26% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 41 | 0.26% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 42 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 43 | 0.26% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.compareAndSetState(int, int) line: 569` |
| 44 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 45 | 0.22% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 46 | 0.22% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 85` |
| 47 | 0.22% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 48 | 0.22% | `java.util.HashMap.putVal(int, Object, Object, boolean, boolean) line: 641` |
| 49 | 0.22% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 50 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.markUnsaved() line: 241` |
| 51 | 0.22% | `java.util.ArrayList$Itr.next() line: 1054` |
| 52 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 53 | 0.18% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 54 | 0.18% | `java.util.HashMap.computeIfAbsent(Object, Function) line: 1219` |
| 55 | 0.18% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 56 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 57 | 0.18% | `net.minecraft.world.level.LevelHeightAccessor.getSectionIndexFromSectionY(int) line: 44` |
| 58 | 0.18% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 59 | 0.18% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 60 | 0.18% | `net.minecraft.nbt.ListTag.identifyRawElementType() line: 202` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 7.46% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 5.42% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 4 | 5.23% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 3.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 6 | 3.55% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 7 | 3.33% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 8 | 2.82% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 9 | 2.74% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 10 | 1.90% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.87% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 12 | 1.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.72% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 14 | 1.57% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 15 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 16 | 1.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 1.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 19 | 1.10% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 20 | 1.02% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 21 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 22 | 0.84% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 23 | 0.73% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 24 | 0.70% | `net.minecraft.world.level.Level.getBlockState(BlockPos) line: 1340` |
| 25 | 0.70% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 26 | 0.66% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 27 | 0.59% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 28 | 0.55% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 30 | 0.48% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |

GC pauses: 0, total 0 ms


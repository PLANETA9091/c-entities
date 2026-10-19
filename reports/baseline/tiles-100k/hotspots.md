# JVM hotspots (JFR ExecutionSample)

Samples: 2917

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.26% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 4.97% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 3 | 4.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 3.77% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 3.46% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 3.29% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 3.22% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 3.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 2.23% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 2.09% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.78% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 13 | 1.75% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 14 | 1.68% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 15 | 1.54% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 16 | 1.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 18 | 1.30% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 19 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 1.17% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 22 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 23 | 1.03% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 24 | 0.69% | `java.lang.String.hashCode() line: 2358` |
| 25 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 26 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 27 | 0.55% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 28 | 0.48% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 30 | 0.45% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 31 | 0.45% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 32 | 0.41% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 33 | 0.41% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 34 | 0.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 35 | 0.41% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 36 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.38% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 38 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 39 | 0.38% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.isLit() line: 152` |
| 40 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 41 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 43 | 0.34% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 44 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.31% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 46 | 0.31% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 47 | 0.31% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 48 | 0.31% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 49 | 0.31% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 50 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 51 | 0.27% | `java.lang.String.equals(Object) line: 1850` |
| 52 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 53 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 54 | 0.27% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 55 | 0.27% | `com.google.common.collect.Iterators.forArrayWithPosition(Object[], int) line: 1075` |
| 56 | 0.27% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 57 | 0.24% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 58 | 0.24% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 59 | 0.24% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.<init>(int, float) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.11% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 3 | 3.98% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 3.43% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 5 | 3.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 6 | 3.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 7 | 3.12% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 8 | 2.95% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 9 | 2.09% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 2.09% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 11 | 2.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.89% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 13 | 1.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 14 | 1.78% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 15 | 1.75% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 16 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.58% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 18 | 1.54% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 19 | 1.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 1.37% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 22 | 1.34% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 23 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 24 | 1.17% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 25 | 1.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 26 | 0.93% | `net.minecraft.world.level.Level.getBlockState(BlockPos) line: 1340` |
| 27 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 28 | 0.82% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 29 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 30 | 0.62% | `net.minecraft.world.level.block.AbstractFurnaceBlock.lambda$createFurnaceTicker$0(ServerLevel, Level, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 90` |

GC pauses: 0, total 0 ms


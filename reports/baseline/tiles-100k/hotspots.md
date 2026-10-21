# JVM hotspots (JFR ExecutionSample)

Samples: 3661

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.49% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 5.38% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 5.11% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.76% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 1.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 8 | 1.78% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 1.69% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.69% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.56% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 13 | 1.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 14 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 15 | 1.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.39% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 17 | 1.23% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 18 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.79% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 20 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.55% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 23 | 0.55% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 24 | 0.52% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 25 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 26 | 0.49% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 27 | 0.46% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 28 | 0.46% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 29 | 0.44% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 30 | 0.44% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 31 | 0.44% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 32 | 0.44% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 33 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 34 | 0.44% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 35 | 0.38% | `java.lang.String.hashCode() line: 2358` |
| 36 | 0.38% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 37 | 0.38% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 38 | 0.36% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 39 | 0.36% | `net.minecraft.core.BlockPos.relative(Direction) line: 230` |
| 40 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.33% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 42 | 0.33% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 43 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 228` |
| 44 | 0.33% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 45 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 46 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 47 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 48 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.30% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 50 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 51 | 0.27% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 52 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 53 | 0.27% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 54 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 55 | 0.27% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 185` |
| 56 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.25% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 59 | 0.25% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.updateForZ(double) line: 737` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.isSafeToUnload() line: 713` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.82% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.22% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 3 | 4.56% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 4 | 3.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 5 | 3.00% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 6 | 2.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 7 | 2.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.29% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 9 | 2.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.78% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 11 | 1.72% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 12 | 1.72% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 13 | 1.64% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 14 | 1.39% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 15 | 1.23% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 16 | 1.17% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 17 | 1.01% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 18 | 0.98% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 19 | 0.98% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 20 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 23 | 0.82% | `net.minecraft.world.level.Level.updateNeighbourForOutputSignal(BlockPos, Block) line: 1978` |
| 24 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 25 | 0.60% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 26 | 0.57% | `net.minecraft.world.level.block.AbstractFurnaceBlock.lambda$createFurnaceTicker$0(ServerLevel, Level, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 90` |
| 27 | 0.55% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 28 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 29 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 30 | 0.52% | `net.minecraft.world.level.block.state.StateHolder.getValue(Property) line: 103` |

GC pauses: 0, total 0 ms


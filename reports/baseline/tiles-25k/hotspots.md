# JVM hotspots (JFR ExecutionSample)

Samples: 1700

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 2.88% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 3 | 2.82% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 2.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 2.41% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 7 | 2.29% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 2.12% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.94% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 10 | 1.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.18% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.00% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 16 | 0.94% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 17 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 18 | 0.88% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 19 | 0.82% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 20 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 22 | 0.65% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 23 | 0.65% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 24 | 0.65% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 27 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 28 | 0.59% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 29 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 30 | 0.53% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 31 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 32 | 0.53% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 33 | 0.53% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 34 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 36 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 37 | 0.47% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 38 | 0.47% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 39 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 40 | 0.41% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 41 | 0.41% | `java.lang.String.hashCode() line: 2358` |
| 42 | 0.41% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 43 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 44 | 0.41% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 45 | 0.41% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 46 | 0.41% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 47 | 0.35% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 48 | 0.35% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 49 | 0.35% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 50 | 0.35% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 51 | 0.35% | `net.minecraft.world.level.levelgen.SurfaceRules$LazyCondition.test() line: 474` |
| 52 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 53 | 0.35% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 54 | 0.35% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 55 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 56 | 0.35% | `net.minecraft.world.level.border.WorldBorder.isWithinBounds(BlockPos) line: 37` |
| 57 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 58 | 0.29% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 59 | 0.29% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 60 | 0.29% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.47% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 3 | 2.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 4 | 2.59% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 5 | 2.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 6 | 2.29% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 7 | 2.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 8 | 2.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.06% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 2.00% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 11 | 1.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 12 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 13 | 1.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 14 | 1.18% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 15 | 1.12% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 16 | 1.06% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 17 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.82% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 20 | 0.82% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 21 | 0.71% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 22 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 23 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |
| 24 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 536` |
| 25 | 0.65% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 26 | 0.65% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 27 | 0.59% | `net.minecraft.world.level.levelgen.NoiseChunk$NoiseInterpolator.compute(DensityFunction$FunctionContext) line: 748` |
| 28 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 29 | 0.59% | `net.minecraft.world.level.block.state.StateHolder.getValue(Property) line: 103` |
| 30 | 0.59% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |

GC pauses: 0, total 0 ms


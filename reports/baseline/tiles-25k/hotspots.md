# JVM hotspots (JFR ExecutionSample)

Samples: 2034

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 2 | 4.03% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 3.00% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 4 | 2.75% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.36% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.21% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 7 | 2.21% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 2.06% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.72% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 10 | 1.67% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.67% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 14 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.13% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 16 | 0.98% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 17 | 0.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.98% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 19 | 0.93% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 20 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 22 | 0.84% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 23 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 24 | 0.79% | `java.lang.String.hashCode() line: 2358` |
| 25 | 0.74% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 26 | 0.69% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 27 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 381` |
| 28 | 0.64% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 30 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.59% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 32 | 0.59% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 33 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 34 | 0.54% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 35 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 36 | 0.54% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 37 | 0.49% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 38 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 39 | 0.44% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 40 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 41 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 42 | 0.39% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 43 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.39% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 45 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 46 | 0.39% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 47 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 50 | 0.34% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 51 | 0.34% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 52 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 55 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.29% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.markUnsaved() line: 241` |
| 60 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 3 | 3.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 3.15% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 2.46% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 6 | 2.21% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 7 | 1.82% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 8 | 1.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.67% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 10 | 1.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 12 | 1.52% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 13 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 14 | 1.18% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 15 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 16 | 1.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 17 | 1.03% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 18 | 0.98% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 19 | 0.98% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 20 | 0.98% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 21 | 0.93% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.84% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 24 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 25 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 26 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 27 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 28 | 0.74% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.74% | `java.lang.AbstractStringBuilder.append(char) line: 811` |
| 30 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |

GC pauses: 0, total 0 ms


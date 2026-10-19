# JVM hotspots (JFR ExecutionSample)

Samples: 3310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.76% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.71% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 4.05% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 4.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 2.99% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 2.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.60% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.02% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 10 | 1.72% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.69% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 14 | 1.33% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 15 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 16 | 1.00% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 17 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 97` |
| 19 | 0.94% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 20 | 0.94% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 23 | 0.76% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 24 | 0.76% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 25 | 0.69% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 26 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.60% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 28 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.48% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 30 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 31 | 0.48% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 32 | 0.48% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 33 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 34 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.45% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 36 | 0.42% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 37 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.42% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 39 | 0.39% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 40 | 0.39% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 41 | 0.39% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 42 | 0.39% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 43 | 0.36% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 44 | 0.36% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 45 | 0.36% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 46 | 0.36% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 47 | 0.36% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 48 | 0.33% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 49 | 0.33% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 50 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.33% | `net.minecraft.world.level.Level.updateNeighbourForOutputSignal(BlockPos, Block) line: 1977` |
| 52 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 53 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 388` |
| 54 | 0.30% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 55 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 56 | 0.30% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 57 | 0.27% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 164` |
| 58 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 59 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.27% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.59% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.38% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 2.96% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 6 | 2.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 7 | 2.60% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 8 | 2.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 2.54% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 2.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 11 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 2.05% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 13 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 14 | 1.69% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 15 | 1.60% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 16 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 17 | 1.33% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 18 | 1.09% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 19 | 1.09% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 20 | 1.06% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 21 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 23 | 0.94% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 24 | 0.82% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 25 | 0.73% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 26 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 27 | 0.60% | `net.minecraft.world.level.block.state.StateHolder.getValue(Property) line: 103` |
| 28 | 0.57% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 29 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.48% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 181` |

GC pauses: 0, total 0 ms


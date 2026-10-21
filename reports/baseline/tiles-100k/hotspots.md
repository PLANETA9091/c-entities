# JVM hotspots (JFR ExecutionSample)

Samples: 3527

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.13% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 2 | 8.85% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 4.37% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.00% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.98% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.64% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 8 | 2.52% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 9 | 2.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.79% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 11 | 1.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.50% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 13 | 1.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 14 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 18 | 1.11% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 19 | 0.94% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 20 | 0.77% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 21 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.62% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 24 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 25 | 0.60% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 26 | 0.57% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 27 | 0.57% | `com.google.common.collect.Iterators.forArrayWithPosition(Object[], int) line: 1075` |
| 28 | 0.54% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 29 | 0.51% | `java.lang.String.hashCode() line: 2358` |
| 30 | 0.51% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 31 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 32 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.40% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 34 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 35 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 36 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.37% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 38 | 0.34% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 39 | 0.34% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 40 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 41 | 0.31% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 42 | 0.31% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 43 | 0.31% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 44 | 0.28% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 45 | 0.28% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 46 | 0.28% | `net.minecraft.core.BlockPos.relative(Direction) line: 234` |
| 47 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 48 | 0.26% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 49 | 0.26% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 50 | 0.26% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 51 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 52 | 0.26% | `net.minecraft.core.BlockPos.relative(Direction) line: 232` |
| 53 | 0.23% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 54 | 0.23% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 55 | 0.23% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 56 | 0.23% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 57 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 58 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 59 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 60 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 7.14% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 4.71% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 2.81% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 2.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 6 | 2.64% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 7 | 2.55% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 8 | 2.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 9 | 2.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 10 | 2.10% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.79% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 13 | 1.64% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 14 | 1.62% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 16 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.22% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 18 | 1.13% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 19 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 20 | 0.96% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 21 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.85% | `net.minecraft.world.level.Level.updateNeighbourForOutputSignal(BlockPos, Block) line: 1978` |
| 23 | 0.77% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 24 | 0.77% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 25 | 0.71% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 26 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 27 | 0.60% | `net.minecraft.world.level.ChunkPos.asLong(BlockPos) line: 89` |
| 28 | 0.57% | `com.google.common.collect.Iterators.forArray(Object[]) line: 1059` |
| 29 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.isTicking(BlockPos) line: 522` |
| 30 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 1525

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.92% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 6.56% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 5.51% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 4 | 5.31% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 3.15% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 3.02% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.82% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.97% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.51% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.38% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 13 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.25% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 16 | 1.18% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 17 | 1.18% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 18 | 1.05% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 19 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 22 | 0.79% | `java.lang.String.hashCode() line: 2358` |
| 23 | 0.79% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 24 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.72% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 26 | 0.72% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 28 | 0.59% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.52% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 30 | 0.46% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 31 | 0.46% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 32 | 0.46% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 33 | 0.46% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 34 | 0.46% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 35 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1312` |
| 36 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.39% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 38 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 39 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 40 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 138` |
| 41 | 0.33% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 42 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 43 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 44 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.keySet() line: 1119` |
| 45 | 0.33% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 46 | 0.33% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |
| 47 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 48 | 0.26% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 49 | 0.26% | `java.util.concurrent.ConcurrentHashMap.transfer(ConcurrentHashMap$Node[], ConcurrentHashMap$Node[]) line: 2489` |
| 50 | 0.26% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 51 | 0.26% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 52 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getMinY() line: 482` |
| 53 | 0.26% | `com.google.common.base.Suppliers$NonSerializableMemoizingSupplier.get() line: 200` |
| 54 | 0.26% | `it.unimi.dsi.fastutil.ints.IntArraySet.findKey(int) line: 205` |
| 55 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 56 | 0.26% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 85` |
| 57 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 60 | 0.26% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.70% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 3 | 5.31% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 4 | 3.02% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 2.89% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 6 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 7 | 1.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 8 | 1.97% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 9 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.64% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 12 | 1.51% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 13 | 1.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 14 | 1.44% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 15 | 1.25% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 16 | 1.25% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 17 | 1.25% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 18 | 1.18% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 19 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 20 | 1.05% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 21 | 1.05% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.85% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 25 | 0.79% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 26 | 0.72% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, int, int, Object) line: 212` |
| 28 | 0.72% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 29 | 0.72% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 30 | 0.59% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |

GC pauses: 0, total 0 ms


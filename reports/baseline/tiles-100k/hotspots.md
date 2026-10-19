# JVM hotspots (JFR ExecutionSample)

Samples: 2787

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.02% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 9.76% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 6.57% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 6.03% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.16% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 3.16% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 3.01% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 9 | 1.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 11 | 1.54% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 12 | 1.47% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 13 | 1.33% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 14 | 1.26% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 15 | 1.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 16 | 1.18% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 17 | 1.15% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 18 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 1.11% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 20 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.93% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 22 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 23 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.61% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 25 | 0.57% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 26 | 0.54% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 27 | 0.54% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 28 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 29 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.markUnsaved() line: 241` |
| 30 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 31 | 0.39% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 32 | 0.36% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 33 | 0.36% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 34 | 0.36% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 35 | 0.36% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 36 | 0.32% | `net.minecraft.world.level.block.entity.BlockEntity.getLevel() line: 90` |
| 37 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 38 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 39 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 40 | 0.32% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 41 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 42 | 0.29% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 43 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 44 | 0.29% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |
| 45 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.25% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 47 | 0.25% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 48 | 0.25% | `java.lang.ThreadLocal.get() line: 172` |
| 49 | 0.25% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 50 | 0.25% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 51 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 52 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 53 | 0.22% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 54 | 0.22% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 55 | 0.22% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 56 | 0.22% | `com.google.common.collect.AbstractIndexedListIterator.hasNext() line: 73` |
| 57 | 0.22% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 58 | 0.22% | `net.minecraft.server.level.ServerLevel.getChunkIfLoaded(int, int) line: 220` |
| 59 | 0.22% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 499` |
| 60 | 0.22% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 185` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 8.14% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 6.28% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 3.80% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 5 | 3.16% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 6 | 3.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 7 | 3.09% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 8 | 2.22% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 9 | 2.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 10 | 1.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 11 | 1.83% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.69% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 13 | 1.61% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 14 | 1.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.47% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 16 | 1.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 17 | 1.18% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 18 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 19 | 0.93% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 20 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.93% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 22 | 0.83% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 23 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 24 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 25 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 26 | 0.61% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 27 | 0.50% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.47% | `net.minecraft.world.level.Level.blockEntityChanged(BlockPos) line: 1860` |
| 29 | 0.47% | `net.minecraft.world.level.ChunkPos.asLong(BlockPos) line: 89` |
| 30 | 0.43% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |

GC pauses: 0, total 0 ms


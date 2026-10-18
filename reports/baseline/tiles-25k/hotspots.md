# JVM hotspots (JFR ExecutionSample)

Samples: 1507

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 3 | 2.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.19% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 2.06% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 6 | 2.06% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 7 | 2.06% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 8 | 1.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.86% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 12 | 1.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.13% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.06% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 16 | 0.93% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 17 | 0.86% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 18 | 0.80% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 19 | 0.80% | `java.lang.String.hashCode() line: 2358` |
| 20 | 0.80% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.73% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 22 | 0.73% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 23 | 0.73% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 24 | 0.73% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 25 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 26 | 0.66% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 30 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 31 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.60% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 33 | 0.60% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 34 | 0.60% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 36 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 37 | 0.53% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 38 | 0.53% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 39 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 40 | 0.53% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 41 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 42 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 43 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 44 | 0.53% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 45 | 0.46% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 46 | 0.46% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 47 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 48 | 0.46% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 49 | 0.46% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 50 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 51 | 0.46% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 52 | 0.46% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 53 | 0.46% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 54 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.40% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 378` |
| 56 | 0.40% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 57 | 0.40% | `net.minecraft.world.level.levelgen.SurfaceRules$BiomeConditionSource$1BiomeCondition.compute() line: 188` |
| 58 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 59 | 0.40% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 60 | 0.40% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.31% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 3.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 5 | 2.46% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 6 | 2.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 7 | 2.06% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 8 | 2.06% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 9 | 1.86% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 10 | 1.79% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 11 | 1.66% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 12 | 1.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 1.39% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 14 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.13% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 16 | 1.13% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 17 | 1.06% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 18 | 1.06% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 19 | 1.06% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 20 | 1.00% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 21 | 0.86% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 25 | 0.73% | `java.util.regex.Matcher.appendReplacement(StringBuffer, String) line: 951` |
| 26 | 0.73% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 27 | 0.73% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 28 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$KeyIterator.next() line: 1042` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 30 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |

GC pauses: 0, total 0 ms


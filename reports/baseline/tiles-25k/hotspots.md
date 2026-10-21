# JVM hotspots (JFR ExecutionSample)

Samples: 1552

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.99% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.41% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.06% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 2.90% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.64% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 1.87% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.48% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.42% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 11 | 1.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.29% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.29% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 15 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 1.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 1.10% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 20 | 0.77% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 21 | 0.77% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1123` |
| 22 | 0.77% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 23 | 0.71% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 24 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 25 | 0.71% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 26 | 0.71% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 27 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 28 | 0.64% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 29 | 0.64% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 30 | 0.58% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 31 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 32 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.52% | `java.lang.String.hashCode() line: 2358` |
| 35 | 0.52% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 36 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 37 | 0.45% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 38 | 0.45% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 39 | 0.39% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 40 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.39% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 43 | 0.39% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 45 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 46 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 47 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 48 | 0.32% | `java.util.HashMap.getNode(Object) line: 587` |
| 49 | 0.32% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 50 | 0.32% | `net.minecraft.world.level.levelgen.synth.PerlinNoise.wrap(double) line: 190` |
| 51 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 52 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 53 | 0.32% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 54 | 0.32% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 55 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.26% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |
| 57 | 0.26% | `net.neoforged.art.internal.EnhancedRemapper.map(String) line: 88` |
| 58 | 0.26% | `java.lang.String.equals(Object) line: 1850` |
| 59 | 0.26% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean) line: 1823` |
| 60 | 0.26% | `java.util.HashMap$HashIterator.nextNode() line: 1609` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.15% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.96% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 2.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 2.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 7 | 2.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 8 | 1.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 9 | 1.87% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 10 | 1.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 11 | 1.55% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 12 | 1.42% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.35% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 15 | 1.29% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 16 | 1.29% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 17 | 1.29% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 18 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 19 | 0.90% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 20 | 0.90% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 21 | 0.84% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 23 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 24 | 0.77% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 25 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 26 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 27 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 28 | 0.71% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 29 | 0.64% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |

GC pauses: 0, total 0 ms


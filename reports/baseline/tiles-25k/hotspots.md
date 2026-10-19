# JVM hotspots (JFR ExecutionSample)

Samples: 1533

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.07% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 2 | 9.00% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 3 | 7.18% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 5.02% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.37% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.61% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 8 | 1.70% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 9 | 1.70% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 10 | 1.50% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 11 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 13 | 1.30% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.17% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 16 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 19 | 0.98% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 20 | 0.91% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 0.85% | `net.minecraft.world.level.chunk.SingleValuePalette.idFor(Object) line: 45` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 23 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 24 | 0.72% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 25 | 0.72% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 26 | 0.72% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 27 | 0.72% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 28 | 0.65% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 29 | 0.59% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 30 | 0.52% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 185` |
| 31 | 0.46% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 32 | 0.46% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 33 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 34 | 0.39% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 35 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 36 | 0.39% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 37 | 0.39% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 38 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 39 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 40 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 41 | 0.33% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 42 | 0.33% | `java.lang.String.hashCode() line: 2358` |
| 43 | 0.33% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 44 | 0.33% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 45 | 0.33% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 46 | 0.33% | `net.minecraft.world.level.levelgen.NoiseChunk.fillAllDirectly(double[], DensityFunction) line: 277` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 48 | 0.33% | `net.minecraft.world.level.Level.updateNeighbourForOutputSignal(BlockPos, Block) line: 1977` |
| 49 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 50 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 51 | 0.26% | `java.io.ByteArrayInputStream.read() line: 147` |
| 52 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.26% | `java.util.regex.Matcher.appendExpandedReplacement(Appendable, String) line: 1142` |
| 54 | 0.26% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 55 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 56 | 0.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 57 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.isRedstoneConductor(BlockGetter, BlockPos) line: 734` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 60 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.61% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 5.15% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 4.50% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 2.61% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 6 | 2.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.02% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 9 | 1.76% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 10 | 1.63% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 11 | 1.50% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 12 | 1.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 13 | 1.44% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 14 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 15 | 1.30% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 16 | 1.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 17 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.24% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 19 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 20 | 1.17% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 21 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 22 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.85% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 24 | 0.72% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.isRemoved() line: 956` |
| 26 | 0.65% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 27 | 0.65% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 28 | 0.59% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.59% | `net.minecraft.world.level.block.AbstractFurnaceBlock.lambda$createFurnaceTicker$0(ServerLevel, Level, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 90` |
| 30 | 0.52% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |

GC pauses: 0, total 0 ms


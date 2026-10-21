# JVM hotspots (JFR ExecutionSample)

Samples: 1651

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.18% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.63% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.48% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.45% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 3.27% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 6 | 2.73% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.00% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.88% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.57% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 12 | 1.45% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 14 | 1.45% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 17 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 18 | 0.97% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 19 | 0.91% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 20 | 0.91% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 21 | 0.85% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 23 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 25 | 0.67% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 26 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 28 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.61% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 30 | 0.61% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 31 | 0.61% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 32 | 0.55% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.p(int) line: 82` |
| 33 | 0.55% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 35 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 36 | 0.48% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 37 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 38 | 0.42% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 39 | 0.42% | `java.lang.String.hashCode() line: 2358` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 41 | 0.42% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 42 | 0.42% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 137` |
| 43 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 44 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 45 | 0.42% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 46 | 0.42% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.compareAndSetState(int, int) line: 569` |
| 47 | 0.42% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 48 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 49 | 0.36% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 50 | 0.36% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 51 | 0.36% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.isLit() line: 152` |
| 52 | 0.36% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 53 | 0.30% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 54 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 55 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 57 | 0.30% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 58 | 0.24% | `paperclip.libs.io.sigpipe.jbsdiff.Patch.patch(byte[], byte[], OutputStream) line: 94` |
| 59 | 0.24% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 60 | 0.24% | `java.util.regex.Pattern$CharPropertyGreedy.match(Matcher, int, CharSequence) line: 4452` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.30% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 3.63% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.51% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 2.73% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 5 | 2.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 7 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 8 | 1.94% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 9 | 1.57% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 10 | 1.45% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 11 | 1.45% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 12 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 13 | 1.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 14 | 1.45% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 15 | 1.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.21% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 18 | 1.09% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 19 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 20 | 1.03% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 21 | 1.03% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 22 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.91% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 24 | 0.91% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 25 | 0.85% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 26 | 0.85% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 27 | 0.79% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 28 | 0.79% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 29 | 0.73% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.67% | `java.lang.String.hashCode() line: 2358` |

GC pauses: 0, total 0 ms


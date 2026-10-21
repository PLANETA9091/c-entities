# JVM hotspots (JFR ExecutionSample)

Samples: 2150

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.98% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 4.65% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 3.12% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 4 | 2.70% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 6 | 1.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.95% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 8 | 1.77% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 9 | 1.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 11 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 12 | 1.16% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.07% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 16 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.93% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 19 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 20 | 0.79% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 21 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 22 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 23 | 0.74% | `java.lang.String.hashCode() line: 2358` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 26 | 0.65% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 27 | 0.65% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 28 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 29 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 31 | 0.56% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 32 | 0.56% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 33 | 0.51% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 34 | 0.51% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 35 | 0.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 36 | 0.51% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 37 | 0.47% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |
| 38 | 0.47% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 39 | 0.47% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 40 | 0.47% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 41 | 0.47% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 42 | 0.47% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 43 | 0.42% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 44 | 0.42% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 45 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 46 | 0.42% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 47 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.37% | `java.lang.String.equals(Object) line: 1850` |
| 49 | 0.37% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 50 | 0.37% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 51 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 52 | 0.37% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 53 | 0.37% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 54 | 0.37% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 55 | 0.37% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 56 | 0.33% | `org.objectweb.asm.ClassReader.readUtf(int, int, char[]) line: 3733` |
| 57 | 0.33% | `org.objectweb.asm.ByteVector.putUTF8(String) line: 268` |
| 58 | 0.33% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 59 | 0.28% | `java.util.HashMap.getNode(Object) line: 585` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.setBlockState(BlockPos, BlockState) line: 199` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 3.91% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 4 | 2.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 6 | 2.00% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 7 | 1.95% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 8 | 1.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 9 | 1.53% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 10 | 1.49% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 11 | 1.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 12 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 13 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 14 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.16% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 16 | 1.12% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 17 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.07% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 19 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 0.93% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 21 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 22 | 0.84% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.74% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 24 | 0.74% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 25 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 26 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 27 | 0.65% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.65% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.65% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 30 | 0.60% | `net.minecraft.world.level.Level.getBlockState(BlockPos) line: 1340` |

GC pauses: 0, total 0 ms


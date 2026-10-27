# JVM hotspots (JFR ExecutionSample)

Samples: 1197

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.02% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 3.01% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 2.92% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 4 | 2.51% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 2.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 6 | 2.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 7 | 2.01% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 8 | 1.92% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 9 | 1.50% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 10 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.42% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 12 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.34% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 14 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 1.25% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1175` |
| 16 | 1.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 17 | 1.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 18 | 1.09% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 19 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 20 | 1.00% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 0.92% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 22 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 23 | 0.84% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 25 | 0.67% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 129` |
| 27 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 29 | 0.58% | `java.lang.String.hashCode() line: 2358` |
| 30 | 0.58% | `java.util.HashMap.getNode(Object) line: 585` |
| 31 | 0.50% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 32 | 0.50% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 33 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 34 | 0.50% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 35 | 0.50% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 36 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 37 | 0.42% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 38 | 0.42% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 39 | 0.42% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 501` |
| 40 | 0.42% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 41 | 0.42% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 42 | 0.42% | `net.minecraft.world.level.redstone.NeighborUpdater.executeUpdate(Level, BlockState, BlockPos, Block, Orientation, boolean, BlockPos) line: 63` |
| 43 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.42% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 45 | 0.42% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.42% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.compareAndSetState(int, int) line: 569` |
| 47 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 48 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 49 | 0.33% | `java.lang.String.equals(Object) line: 1850` |
| 50 | 0.33% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 51 | 0.33% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 52 | 0.33% | `net.minecraft.world.level.levelgen.NoiseChunk.updateForZ(int, double) line: 325` |
| 53 | 0.33% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.tryPropagateSkylight(BlockGetter, int, int, int, boolean, boolean) line: 614` |
| 55 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 56 | 0.33% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 57 | 0.33% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 59 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 60 | 0.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 2 | 5.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 3 | 2.92% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 4 | 2.51% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 2.51% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 6 | 2.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 9 | 2.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 10 | 2.01% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 11 | 1.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 13 | 1.50% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 14 | 1.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 1.34% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 16 | 1.34% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 17 | 1.17% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 18 | 1.09% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 20 | 1.09% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 21 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.lambda$codec$6(IdMap, PalettedContainer$Strategy, PalettedContainerRO) line: 73` |
| 22 | 0.84% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.84% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 24 | 0.75% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.onResize(int, Object) line: 191` |
| 26 | 0.67% | `net.minecraft.world.level.levelgen.SurfaceSystem.buildSurface(RandomState, BiomeManager, Registry, boolean, WorldGenerationContext, ChunkAccess, NoiseChunk, SurfaceRules$RuleSource) line: 150` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.setBlockState(int, int, int, BlockState, boolean) line: 167` |
| 28 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.checkNeighbour(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 805` |
| 29 | 0.67% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 30 | 0.67% | `net.minecraft.world.level.Level.updateNeighbourForOutputSignal(BlockPos, Block) line: 1978` |

GC pauses: 0, total 0 ms


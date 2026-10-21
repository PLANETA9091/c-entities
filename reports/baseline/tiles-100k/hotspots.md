# JVM hotspots (JFR ExecutionSample)

Samples: 3119

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 7.89% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 5.03% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 4.65% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 4.52% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 2.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.79% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 8 | 2.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 9 | 2.02% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 10 | 1.99% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 11 | 1.83% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 12 | 1.67% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 13 | 1.64% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 14 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.54% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 16 | 1.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 1.25% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 18 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.99% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 20 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.93% | `java.util.Iterator.forEachRemaining(Consumer) line: 133` |
| 22 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 23 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 24 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.58% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 26 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.51% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 28 | 0.51% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 29 | 0.42% | `com.google.common.collect.Iterators.forArrayWithPosition(Object[], int) line: 1075` |
| 30 | 0.38% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 31 | 0.38% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 32 | 0.38% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.isLit() line: 152` |
| 33 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 34 | 0.35% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 35 | 0.35% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 36 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 37 | 0.35% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 38 | 0.35% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 39 | 0.32% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 40 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 41 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 42 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.getBlockState(int, int, int) line: 77` |
| 43 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 44 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 45 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 46 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 47 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 48 | 0.29% | `net.minecraft.server.level.ServerLevel.areEntitiesLoaded(long) line: 2516` |
| 49 | 0.29% | `net.minecraft.world.level.redstone.NeighborUpdater.executeUpdate(Level, BlockState, BlockPos, Block, Orientation, boolean, BlockPos) line: 63` |
| 50 | 0.29% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.<init>(int, float) line: 96` |
| 53 | 0.26% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 54 | 0.26% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 55 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 56 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 57 | 0.22% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 59 | 0.19% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 60 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 6.51% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 4.75% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 4 | 3.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 5 | 2.79% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 6 | 2.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 7 | 2.60% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 8 | 2.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 9 | 2.12% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 10 | 2.05% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 11 | 2.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.83% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 13 | 1.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 14 | 1.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.67% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 16 | 1.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 17 | 1.25% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 18 | 1.19% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 19 | 1.12% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 20 | 1.09% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 21 | 0.99% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 22 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 23 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.93% | `net.minecraft.world.level.chunk.HashMapPalette.getEntries() line: 117` |
| 25 | 0.71% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 26 | 0.55% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 27 | 0.55% | `net.minecraft.world.level.block.AbstractFurnaceBlock.lambda$createFurnaceTicker$0(ServerLevel, Level, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 90` |
| 28 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.51% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 30 | 0.48% | `net.minecraft.nbt.ListTag.write(DataOutput) line: 194` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 2552

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.79% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 7.95% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 7.05% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 6.31% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 4.47% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 6 | 3.41% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 2.59% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 8 | 2.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 9 | 2.12% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 10 | 1.92% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 11 | 1.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.61% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 13 | 1.25% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 14 | 1.25% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 15 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 16 | 0.98% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 17 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 20 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 23 | 0.63% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 24 | 0.59% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 26 | 0.47% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 27 | 0.47% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 28 | 0.43% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 29 | 0.39% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 30 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 31 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 32 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 33 | 0.31% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 34 | 0.31% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 35 | 0.31% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 36 | 0.31% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 37 | 0.31% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 38 | 0.31% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1144` |
| 40 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.markUnsaved() line: 241` |
| 41 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 42 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 43 | 0.27% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 44 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 45 | 0.27% | `net.minecraft.world.RandomizableContainer.unpackLootTable(Player, boolean) line: 84` |
| 46 | 0.27% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 47 | 0.24% | `java.lang.String.equals(Object) line: 1850` |
| 48 | 0.24% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 49 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 50 | 0.24% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 51 | 0.24% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 52 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 53 | 0.24% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 54 | 0.24% | `net.minecraft.world.level.block.entity.BlockEntityType.getBlockEntity(BlockGetter, BlockPos) line: 277` |
| 55 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 56 | 0.20% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 30` |
| 57 | 0.20% | `java.util.ImmutableCollections$Set12.contains(Object) line: 817` |
| 58 | 0.20% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1139` |
| 59 | 0.20% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 60 | 0.20% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 8.07% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 7.92% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 4 | 4.74% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 3.72% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 6 | 3.53% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 7 | 2.59% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 8 | 2.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 9 | 2.16% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 10 | 2.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 11 | 1.96% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 12 | 1.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 1.25% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 16 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.98% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 18 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 19 | 0.94% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 20 | 0.86% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 21 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 22 | 0.78% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 23 | 0.67% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 24 | 0.63% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 25 | 0.59% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 26 | 0.47% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 181` |
| 27 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.43% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.43% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 30 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 3410

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.98% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.84% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 4.13% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 3.58% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 5 | 3.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.67% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 1.99% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 8 | 1.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.85% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 10 | 1.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 11 | 1.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 12 | 1.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 13 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 14 | 1.35% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 15 | 1.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 16 | 1.17% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 17 | 1.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 1.06% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 19 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 20 | 0.97% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 21 | 0.94% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 22 | 0.94% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 23 | 0.76% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 24 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.65% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 26 | 0.62% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 27 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.59% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 29 | 0.53% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 30 | 0.53% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 31 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.50% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 33 | 0.50% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 35 | 0.44% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 36 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.44% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 38 | 0.44% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 39 | 0.41% | `java.lang.String.hashCode() line: 2358` |
| 40 | 0.41% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 41 | 0.41% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 42 | 0.41% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.isLit() line: 152` |
| 43 | 0.38% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 44 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.35% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 46 | 0.35% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 47 | 0.32% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 48 | 0.32% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 49 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 50 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 52 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 53 | 0.29% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 54 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.29% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 56 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 57 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 58 | 0.29% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 59 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 60 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 3.31% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 4 | 2.87% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 2.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 6 | 2.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.76% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 8 | 2.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 9 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.91% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 11 | 1.91% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 12 | 1.76% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 13 | 1.50% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 14 | 1.44% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 15 | 1.35% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 16 | 1.32% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 17 | 1.17% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 18 | 1.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 1.11% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 20 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 21 | 1.06% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 1.00% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.97% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 24 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.94% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 26 | 0.88% | `net.minecraft.world.level.block.entity.BlockEntityType.isValid(BlockState) line: 266` |
| 27 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 184` |
| 29 | 0.62% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 30 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |

GC pauses: 0, total 0 ms


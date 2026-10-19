# JVM hotspots (JFR ExecutionSample)

Samples: 1485

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.29% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 8.15% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 8.08% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 4 | 5.52% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 5 | 4.65% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 3.03% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 7 | 1.82% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 8 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 9 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 12 | 1.35% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 14 | 1.14% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1173` |
| 15 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 0.94% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 17 | 0.88% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 18 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 19 | 0.81% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 20 | 0.74% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 23 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 24 | 0.61% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 25 | 0.61% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 26 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 27 | 0.54% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 28 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 29 | 0.54% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 30 | 0.54% | `java.util.Arrays$ArrayList.get(int) line: 4266` |
| 31 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 32 | 0.47% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 33 | 0.47% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 34 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.47% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 36 | 0.47% | `net.minecraft.nbt.CompoundTag.writeNamedTag(String, Tag, DataOutput) line: 433` |
| 37 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.40% | `java.lang.String.hashCode() line: 2358` |
| 39 | 0.40% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 40 | 0.34% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 41 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 42 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.34% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.release(int) line: 1093` |
| 44 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$MapIterator.nextEntry() line: 643` |
| 45 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 46 | 0.27% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 47 | 0.27% | `java.lang.String.equals(Object) line: 1850` |
| 48 | 0.27% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 49 | 0.27% | `java.math.BigInteger.addOne(int[], int, int, int) line: 3355` |
| 50 | 0.27% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1819` |
| 51 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 52 | 0.27% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 53 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 97` |
| 54 | 0.27% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 55 | 0.27% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 56 | 0.27% | `net.minecraft.nbt.CompoundTag.copy() line: 410` |
| 57 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 58 | 0.20% | `java.lang.String.contains(CharSequence) line: 2970` |
| 59 | 0.20% | `org.objectweb.asm.ByteVector.putShort(int) line: 121` |
| 60 | 0.20% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 5.66% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 3 | 5.39% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 4 | 4.71% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 5 | 3.03% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 6 | 2.42% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 7 | 2.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.read0() line: 228` |
| 9 | 1.48% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 10 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.41% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 12 | 1.41% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 13 | 1.35% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 14 | 1.35% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 15 | 1.28% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 16 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 18 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 19 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 21 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 22 | 0.88% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.81% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 586` |
| 24 | 0.81% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 25 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 26 | 0.74% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 28 | 0.67% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 29 | 0.61% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 30 | 0.61% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |

GC pauses: 0, total 0 ms


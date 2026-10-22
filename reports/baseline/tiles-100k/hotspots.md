# JVM hotspots (JFR ExecutionSample)

Samples: 3387

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.30% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.02% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 3 | 4.13% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 3.72% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 5 | 3.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.86% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 2.30% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.10% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 2.07% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 10 | 1.98% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 11 | 1.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 13 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 15 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 1.06% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 17 | 1.00% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 18 | 0.97% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 20 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 21 | 0.92% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 22 | 0.83% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 23 | 0.80% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 24 | 0.77% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 25 | 0.71% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 26 | 0.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.59% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 28 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 29 | 0.44% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 30 | 0.41% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 31 | 0.41% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 32 | 0.41% | `net.minecraft.world.level.block.entity.BlockEntity.isRemoved() line: 261` |
| 33 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 34 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 35 | 0.38% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 36 | 0.38% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 37 | 0.38% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 38 | 0.35% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 39 | 0.35% | `java.lang.String.hashCode() line: 2358` |
| 40 | 0.35% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 41 | 0.35% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 42 | 0.35% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 43 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 44 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 45 | 0.32% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 46 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 47 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 48 | 0.32% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 49 | 0.30% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 50 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 51 | 0.30% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 93` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 53 | 0.27% | `java.util.HashMap$HashIterator.nextNode() line: 1609` |
| 54 | 0.27% | `net.minecraft.world.level.chunk.ProtoChunk.getBlockState(int, int, int) line: 117` |
| 55 | 0.27% | `net.minecraft.world.level.levelgen.DensityFunctions$EndIslandDensityFunction.getHeightValue(SimplexNoise, int, int) line: 554` |
| 56 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 57 | 0.27% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 58 | 0.24% | `java.lang.String.equals(Object) line: 1850` |
| 59 | 0.24% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 60 | 0.24% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 4.84% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 3 | 3.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 4 | 3.10% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 5 | 2.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 6 | 2.86% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 7 | 2.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 8 | 2.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.36% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 10 | 2.07% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 11 | 1.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.83% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 14 | 1.51% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 15 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 16 | 1.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 17 | 1.27% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 18 | 1.18% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 19 | 1.15% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 20 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 1.03% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 0.97% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 23 | 0.94% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 24 | 0.83% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 25 | 0.77% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 26 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 27 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 28 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 29 | 0.53% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 30 | 0.53% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |

GC pauses: 0, total 0 ms


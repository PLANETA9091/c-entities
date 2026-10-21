# JVM hotspots (JFR ExecutionSample)

Samples: 3163

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 2 | 5.12% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 3 | 3.70% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |
| 4 | 3.26% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1461` |
| 5 | 2.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.81% | `net.minecraft.server.level.ServerLevel.moonrise$getChunkTaskScheduler() line: 342` |
| 7 | 2.31% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.74% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 9 | 1.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockEntity(BlockPos, LevelChunk$EntityCreationType) line: 474` |
| 10 | 1.52% | `net.minecraft.core.NonNullList.get(int) line: 44` |
| 11 | 1.49% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 12 | 1.42% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 13 | 1.39% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 14 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 15 | 1.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 16 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 1.17% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 18 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 1.14% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 20 | 1.07% | `net.minecraft.world.level.block.entity.HopperBlockEntity.pushItemsTick(Level, BlockPos, BlockState, HopperBlockEntity) line: 131` |
| 21 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.isRemoved() line: 1004` |
| 22 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 23 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.92% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 26 | 0.89% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 27 | 0.73% | `ca.spottedleaf.moonrise.patches.blockstate_propertyaccess.util.ZeroCollidingReferenceStateTable.get(long, Property) line: 117` |
| 28 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.blocksMotion() line: 651` |
| 29 | 0.60% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 31 | 0.57% | `com.google.common.collect.Iterators$ArrayItr.<init>(Object[], int) line: 1086` |
| 32 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 34 | 0.44% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 35 | 0.44% | `java.lang.String.hashCode() line: 2358` |
| 36 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 930` |
| 37 | 0.44% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 38 | 0.41% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 39 | 0.41% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 40 | 0.38% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 41 | 0.38% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 590` |
| 42 | 0.38% | `net.minecraft.world.level.levelgen.synth.SimplexNoise.dot(int[], double, double, double) line: 56` |
| 43 | 0.38% | `net.minecraft.world.level.ChunkPos.asLong(int, int) line: 85` |
| 44 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 45 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 47 | 0.35% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 48 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 49 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 50 | 0.32% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 51 | 0.32% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 52 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 53 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.32% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 185` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 350` |
| 56 | 0.28% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 57 | 0.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getHash(long) line: 154` |
| 58 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 59 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 60 | 0.25% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 819` |
| 2 | 3.13% | `net.minecraft.server.level.ServerLevel.shouldTickBlocksAt(long) line: 830` |
| 3 | 3.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 4 | 2.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 5 | 2.59% | `net.minecraft.world.level.block.entity.HopperBlockEntity.isFullContainer(Container, Direction) line: 522` |
| 6 | 2.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.37% | `net.minecraft.world.Container.getMaxStackSize(ItemStack) line: 32` |
| 9 | 2.31% | `java.util.LinkedHashMap.get(Object) line: 536` |
| 10 | 2.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 11 | 2.06% | `net.minecraft.world.level.block.entity.HopperBlockEntity.getFullState(HopperBlockEntity) line: 173` |
| 12 | 1.61% | `net.minecraft.world.level.Level.getBlockEntity(BlockPos) line: 1640` |
| 13 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 14 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.42% | `net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity.getItem(int) line: 58` |
| 16 | 1.42% | `java.util.ArrayList.batchRemove(Collection, boolean, int, int) line: 905` |
| 17 | 1.39% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 18 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.read0() line: 228` |
| 19 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk$BoundTickingBlockEntity.tick() line: 929` |
| 20 | 1.14% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 21 | 1.11% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(Block) line: 990` |
| 22 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper.tick() line: 999` |
| 23 | 1.04% | `net.minecraft.world.level.Level.tickBlockEntities() line: 1463` |
| 24 | 0.98% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 25 | 0.95% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightDecrease(LightChunkGetter) line: 1436` |
| 26 | 0.79% | `net.minecraft.world.level.block.state.StateHolder.getValue(Property) line: 103` |
| 27 | 0.76% | `net.minecraft.world.level.block.entity.BaseContainerBlockEntity.getItem(int) line: 127` |
| 28 | 0.70% | `net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity.serverTick(ServerLevel, BlockPos, BlockState, AbstractFurnaceBlockEntity) line: 188` |
| 29 | 0.60% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 86` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |

GC pauses: 0, total 0 ms


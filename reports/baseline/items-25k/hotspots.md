# JVM hotspots (JFR ExecutionSample)

Samples: 4584

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.99% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 3.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 3.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.14% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 9 | 1.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 11 | 1.31% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 13 | 1.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.96% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 16 | 0.96% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 0.96% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 18 | 0.89% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 19 | 0.89% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 20 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 22 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.79% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 24 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.68% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 27 | 0.65% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 28 | 0.63% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 29 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 31 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 33 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 374` |
| 34 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 36 | 0.46% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 37 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.44% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |
| 40 | 0.44% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 41 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 42 | 0.41% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 43 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 44 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 45 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 46 | 0.39% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 47 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 48 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 49 | 0.33% | `java.lang.String.hashCode() line: 2358` |
| 50 | 0.33% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 51 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 53 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 54 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 55 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 56 | 0.28% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.26% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 59 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 4.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 4.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 3.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.14% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 2.01% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.46% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 16 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.read0() line: 228` |
| 17 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.pack(IdMap, PalettedContainer$Strategy) line: 352` |
| 18 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 20 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 21 | 0.94% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 22 | 0.87% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 25 | 0.79% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 26 | 0.74% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 27 | 0.74% | `java.util.ArrayList.toArray() line: 369` |
| 28 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |

GC pauses: 0, total 0 ms


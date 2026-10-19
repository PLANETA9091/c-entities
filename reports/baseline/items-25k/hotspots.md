# JVM hotspots (JFR ExecutionSample)

Samples: 3115

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 2 | 3.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 2.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 2.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 1.54% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 6 | 1.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.41% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 10 | 1.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 1.25% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 12 | 1.19% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 15 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 18 | 1.03% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 19 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.93% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 23 | 0.80% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 24 | 0.80% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 25 | 0.80% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.resizeIncreaseQueue() line: 1060` |
| 26 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.64% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 28 | 0.64% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 30 | 0.58% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 527` |
| 31 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 33 | 0.51% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 34 | 0.51% | `java.lang.String.hashCode() line: 2358` |
| 35 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.48% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 1722` |
| 37 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 38 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 39 | 0.45% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.doFill(Blender, StructureManager, RandomState, ChunkAccess, int, int) line: 378` |
| 40 | 0.45% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 41 | 0.45% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 44 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 228` |
| 45 | 0.42% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 46 | 0.42% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 47 | 0.39% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 57` |
| 48 | 0.39% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 49 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 51 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 52 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.35% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 54 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 55 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 56 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 57 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 58 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.32% | `java.util.concurrent.ConcurrentHashMap.get(Object) line: 946` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 2 | 3.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 3.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 2.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 2.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.64% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 8 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 10 | 1.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.41% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 12 | 1.38% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 13 | 1.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 14 | 1.35% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 1.16% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 17 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 18 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 19 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 20 | 1.03% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 21 | 0.93% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 22 | 0.90% | `java.util.ArrayList.toArray() line: 369` |
| 23 | 0.83% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 25 | 0.80% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1173` |
| 26 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 30 | 0.74% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |

GC pauses: 0, total 0 ms


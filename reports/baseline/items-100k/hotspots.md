# JVM hotspots (JFR ExecutionSample)

Samples: 15735

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.31% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 4.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 7 | 2.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 9 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.22% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 16 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 18 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.95% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 21 | 0.82% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 22 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 23 | 0.73% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 24 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 26 | 0.69% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 28 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 29 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 30 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 31 | 0.55% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 32 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 33 | 0.51% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 36 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.44% | `java.util.EnumMap.get(Object) line: 247` |
| 38 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 39 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 40 | 0.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 41 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 43 | 0.32% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 44 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 45 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 46 | 0.31% | `java.lang.ThreadLocal.get() line: 172` |
| 47 | 0.31% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 48 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 50 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 51 | 0.27% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 52 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 54 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 351` |
| 57 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.24% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 59 | 0.23% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 60 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 6.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 4.29% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 4.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 2.64% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.92% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.53% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 14 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.14% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 18 | 0.97% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 20 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 23 | 0.74% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 26 | 0.67% | `java.util.ArrayList.toArray() line: 369` |
| 27 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 29 | 0.61% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


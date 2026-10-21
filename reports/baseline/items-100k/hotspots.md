# JVM hotspots (JFR ExecutionSample)

Samples: 15907

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 5.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 2.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 15 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.14% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 17 | 1.01% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 18 | 0.99% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 19 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 20 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.84% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 22 | 0.82% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 23 | 0.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 24 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.applyGravity() line: 1848` |
| 26 | 0.62% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 27 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 30 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 31 | 0.53% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 33 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 35 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 36 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 37 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 39 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 40 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 43 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 45 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 46 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 47 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 48 | 0.36% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 49 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 50 | 0.34% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 51 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 54 | 0.29% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 55 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 56 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 57 | 0.27% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 59 | 0.26% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 60 | 0.26% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.19% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 2.27% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.12% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 16 | 1.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 18 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.05% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 20 | 0.99% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 21 | 0.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 24 | 0.76% | `java.util.ArrayList.toArray() line: 369` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 174` |

GC pauses: 0, total 0 ms


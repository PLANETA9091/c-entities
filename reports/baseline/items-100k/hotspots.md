# JVM hotspots (JFR ExecutionSample)

Samples: 15796

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.36% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 4.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 3.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 3.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.51% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 12 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.22% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 18 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 20 | 0.88% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 21 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.74% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 23 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 25 | 0.67% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 27 | 0.63% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 28 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 29 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 31 | 0.58% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 32 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 33 | 0.55% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 36 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 37 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 39 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 41 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 42 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 43 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 44 | 0.38% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 45 | 0.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 46 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 47 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 48 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 49 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 50 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.32% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 52 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 55 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 57 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.23% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 6.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 5.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 4.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 2.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 2.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.71% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.57% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 13 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 16 | 1.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 1.18% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 19 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 23 | 0.80% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.74% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 27 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 28 | 0.70% | `java.util.ArrayList.toArray() line: 369` |
| 29 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms


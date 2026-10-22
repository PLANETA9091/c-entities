# JVM hotspots (JFR ExecutionSample)

Samples: 14146

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.77% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 5.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 3.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 9 | 1.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 10 | 1.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 16 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 17 | 1.12% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 18 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 0.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 21 | 0.86% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 22 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 23 | 0.76% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 24 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 27 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 29 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 30 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 31 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 32 | 0.59% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 33 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 34 | 0.54% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 36 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 37 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 39 | 0.42% | `java.util.ArrayList$Itr.next() line: 1057` |
| 40 | 0.40% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 41 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 42 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 43 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 44 | 0.34% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 46 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 47 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 48 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 49 | 0.32% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 50 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 52 | 0.28% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 53 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 55 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 56 | 0.25% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 57 | 0.25% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 58 | 0.24% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 59 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 60 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 7.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 6.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 5.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 3.15% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 1.93% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 10 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.85% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.12% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.07% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 21 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 25 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 28 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |

GC pauses: 0, total 0 ms


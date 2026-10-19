# JVM hotspots (JFR ExecutionSample)

Samples: 16518

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.44% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 4.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 4.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.22% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 14 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.00% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 18 | 0.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 19 | 0.97% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 20 | 0.96% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 21 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.70% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 24 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 25 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 27 | 0.64% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 28 | 0.63% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 30 | 0.58% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 32 | 0.56% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 34 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.54% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 37 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 38 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 39 | 0.50% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 40 | 0.49% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 42 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 43 | 0.44% | `java.lang.Enum.ordinal() line: 128` |
| 44 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 45 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 46 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 47 | 0.34% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 48 | 0.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 49 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 50 | 0.32% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 51 | 0.31% | `java.util.ArrayList$Itr.next() line: 1057` |
| 52 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 54 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 55 | 0.27% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 56 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.26% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 58 | 0.25% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 59 | 0.25% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 60 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 6.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 6.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.59% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 2.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 10 | 1.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 12 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 15 | 1.23% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 16 | 1.22% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 17 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 19 | 1.02% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 20 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.83% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 26 | 0.74% | `java.util.ArrayList.toArray() line: 369` |
| 27 | 0.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 28 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 30 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1462` |

GC pauses: 0, total 0 ms


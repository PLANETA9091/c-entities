# JVM hotspots (JFR ExecutionSample)

Samples: 15632

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.06% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 7 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 11 | 1.63% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 1.60% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.46% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 16 | 1.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 17 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.11% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 19 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 21 | 0.95% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.92% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 24 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 25 | 0.76% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 26 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 28 | 0.70% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 29 | 0.68% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 30 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 31 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 32 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 33 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 35 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 36 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 37 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 40 | 0.47% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 41 | 0.47% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 42 | 0.47% | `java.util.EnumMap.get(Object) line: 247` |
| 43 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 45 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 46 | 0.42% | `java.util.HashMap.getNode(Object) line: 585` |
| 47 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 49 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 50 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 52 | 0.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 53 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 54 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 55 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 56 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 58 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 25` |
| 60 | 0.29% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 6.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 3.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 3.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 7 | 2.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 2.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 10 | 2.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 1.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 12 | 1.64% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 13 | 1.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.35% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.32% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 17 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 1.19% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 21 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 22 | 1.05% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 23 | 1.04% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 24 | 0.96% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 25 | 0.95% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 26 | 0.94% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 27 | 0.93% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1279` |
| 28 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 165` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 30 | 0.74% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |

GC pauses: 0, total 0 ms


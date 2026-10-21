# JVM hotspots (JFR ExecutionSample)

Samples: 23579

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 6 | 3.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.54% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 2.03% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 10 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 11 | 1.48% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 12 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 17 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.88% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 20 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 21 | 0.78% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 24 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 29 | 0.56% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 30 | 0.54% | `java.lang.Enum.ordinal() line: 128` |
| 31 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 32 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 33 | 0.42% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 34 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 35 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 36 | 0.36% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 37 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 38 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 39 | 0.34% | `java.util.ArrayList$Itr.next() line: 1057` |
| 40 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 41 | 0.34% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 43 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4743` |
| 45 | 0.32% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 46 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 47 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 48 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 49 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 50 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 51 | 0.27% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 53 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 54 | 0.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1008` |
| 55 | 0.24% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |
| 56 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 57 | 0.22% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 58 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 59 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 4.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 4 | 4.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 3.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 3.44% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 7 | 2.54% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 2.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 9 | 2.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 2.03% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 1.97% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 13 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.46% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 16 | 1.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 18 | 0.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 21 | 0.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 22 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.79% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 24 | 0.64% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 25 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.54% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 30 | 0.54% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |

GC pauses: 0, total 0 ms


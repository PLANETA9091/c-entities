# JVM hotspots (JFR ExecutionSample)

Samples: 21267

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 3.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 3.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 6 | 3.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.05% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 10 | 1.84% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 11 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.97% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 17 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.72% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 21 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 23 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.69% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 26 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 27 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 28 | 0.56% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 29 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 30 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 31 | 0.44% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.41% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 34 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 35 | 0.41% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1008` |
| 37 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 39 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 40 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 41 | 0.33% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 44 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 45 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 46 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 47 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 48 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 50 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 51 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 52 | 0.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 129` |
| 55 | 0.23% | `java.util.EnumMap.get(Object) line: 247` |
| 56 | 0.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 57 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 58 | 0.22% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 59 | 0.21% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 60 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 4.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 3.97% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 5 | 3.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 3.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 7 | 2.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 2.29% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 2.05% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.87% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 1.83% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 13 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 14 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 17 | 1.34% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 0.98% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.87% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 23 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 25 | 0.67% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 26 | 0.58% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.56% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 29 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |

GC pauses: 0, total 0 ms


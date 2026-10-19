# JVM hotspots (JFR ExecutionSample)

Samples: 11903

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 6 | 2.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.98% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 9 | 1.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 10 | 1.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 12 | 1.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 14 | 1.21% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 15 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 16 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 18 | 1.10% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 19 | 1.07% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 20 | 1.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 21 | 0.92% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 23 | 0.90% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 24 | 0.87% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 26 | 0.82% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 27 | 0.80% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 29 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 30 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 31 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 35 | 0.49% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 36 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 38 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.41% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 40 | 0.39% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 41 | 0.39% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 42 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 43 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 45 | 0.36% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 25` |
| 46 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 47 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 48 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 49 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |
| 50 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 52 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 54 | 0.27% | `net.minecraft.server.level.ServerLevel.moonrise$areChunksLoaded(int, int, int, int) line: 486` |
| 55 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 56 | 0.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 59 | 0.24% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 60 | 0.24% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 6.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 4.44% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 3.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.35% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 1.98% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 11 | 1.75% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 13 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 17 | 1.19% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 1.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 1.13% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 165` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 22 | 1.03% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1725` |
| 23 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.97% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 25 | 0.96% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 26 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 27 | 0.92% | `java.util.ArrayList.toArray() line: 369` |
| 28 | 0.91% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 29 | 0.90% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 30 | 0.90% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |

GC pauses: 0, total 0 ms


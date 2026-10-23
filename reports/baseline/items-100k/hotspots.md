# JVM hotspots (JFR ExecutionSample)

Samples: 15120

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.25% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 3.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 2.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.85% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 13 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.26% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 16 | 1.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 17 | 1.03% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 18 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.94% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 21 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 22 | 0.90% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 24 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 25 | 0.79% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 26 | 0.76% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 27 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 29 | 0.62% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 32 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 33 | 0.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 34 | 0.51% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 35 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 36 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.50% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 38 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 39 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 449` |
| 40 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 41 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 44 | 0.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 45 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 48 | 0.31% | `java.lang.ThreadLocal.get() line: 172` |
| 49 | 0.31% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 50 | 0.30% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 51 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 53 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 54 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 56 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.26% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 59 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1295` |
| 60 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 6.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 5.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 3.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 7 | 2.85% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 11 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 13 | 1.56% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.46% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 15 | 1.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 1.21% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 17 | 1.17% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 18 | 1.03% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 0.97% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 20 | 0.94% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 22 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.85% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 25 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 26 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.65% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 30 | 0.62% | `java.util.ArrayList.toArray() line: 369` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 14372

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.50% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 14 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 17 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 18 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 19 | 1.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.88% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 22 | 0.80% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 23 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 24 | 0.70% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.68% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.66% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 27 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 31 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 32 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 33 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 36 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 38 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 39 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 41 | 0.46% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 44 | 0.44% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 45 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 46 | 0.42% | `java.util.ArrayList$Itr.next() line: 1057` |
| 47 | 0.41% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 49 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 51 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 52 | 0.35% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 53 | 0.33% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 55 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.26% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 59 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.getTicksFrozen() line: 3632` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 7.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 6.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 4.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.69% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 2.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 10 | 2.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 2.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.96% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.46% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 17 | 1.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 1.04% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 21 | 0.93% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 22 | 0.92% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 23 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.81% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 27 | 0.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 29 | 0.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 30 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


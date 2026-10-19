# JVM hotspots (JFR ExecutionSample)

Samples: 15856

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 2.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.66% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 15 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.08% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 17 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 18 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 0.98% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 20 | 0.92% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 21 | 0.91% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 24 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 27 | 0.72% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 28 | 0.64% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 30 | 0.56% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 31 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 32 | 0.54% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 33 | 0.50% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 34 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 36 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 37 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 39 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 40 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 41 | 0.40% | `java.lang.ThreadLocal.get() line: 172` |
| 42 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 43 | 0.40% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.30% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.30% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 47 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 48 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 49 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 50 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 53 | 0.26% | `java.util.ArrayList$Itr.next() line: 1057` |
| 54 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 56 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 57 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 59 | 0.23% | `java.util.ArrayList.clear() line: 735` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 5.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 3.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.79% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.44% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 2.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.66% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 12 | 1.58% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 20 | 0.97% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 21 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.91% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 25 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 28 | 0.79% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 29 | 0.71% | `java.util.ArrayList.toArray() line: 369` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |

GC pauses: 0, total 0 ms


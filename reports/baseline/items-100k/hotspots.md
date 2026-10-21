# JVM hotspots (JFR ExecutionSample)

Samples: 18062

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 5.08% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 4.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 8 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 10 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 14 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 16 | 1.07% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 17 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 18 | 1.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.94% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 20 | 0.90% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 21 | 0.85% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 22 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 27 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 28 | 0.58% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 30 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 31 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 32 | 0.54% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 33 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 35 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 37 | 0.49% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 39 | 0.44% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 40 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.39% | `java.lang.ThreadLocal.get() line: 172` |
| 44 | 0.38% | `java.util.ArrayList$Itr.next() line: 1057` |
| 45 | 0.38% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 46 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4769` |
| 51 | 0.31% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 194` |
| 52 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 53 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4745` |
| 55 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 56 | 0.28% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 58 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 60 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 7.30% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 6.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 5.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.88% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.59% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 12 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 14 | 1.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 17 | 1.07% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 18 | 1.04% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 22 | 0.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 26 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 27 | 0.72% | `java.util.ArrayList.toArray() line: 369` |
| 28 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 30 | 0.66% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |

GC pauses: 0, total 0 ms


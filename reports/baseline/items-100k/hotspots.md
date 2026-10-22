# JVM hotspots (JFR ExecutionSample)

Samples: 15858

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.73% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 4.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 4.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.04% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 2.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.51% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.21% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 19 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 21 | 0.92% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.78% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 23 | 0.67% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 527` |
| 24 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 25 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 27 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 28 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 29 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 30 | 0.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 31 | 0.59% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 33 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 449` |
| 35 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 37 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.46% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 39 | 0.42% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 42 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 43 | 0.38% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 44 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 45 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 46 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 47 | 0.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 48 | 0.33% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 49 | 0.32% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 52 | 0.30% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 53 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 54 | 0.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 55 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 56 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 57 | 0.26% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 59 | 0.24% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 8.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 6.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 4.69% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.04% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.62% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.57% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 14 | 1.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.25% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 17 | 1.21% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 1.08% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 21 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.92% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 23 | 0.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.90% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 25 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 26 | 0.84% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 28 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |

GC pauses: 0, total 0 ms


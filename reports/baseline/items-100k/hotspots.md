# JVM hotspots (JFR ExecutionSample)

Samples: 15984

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 4.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 4.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 3.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 10 | 1.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 12 | 1.48% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 14 | 1.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 1.26% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 17 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 20 | 0.93% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 21 | 0.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 23 | 0.86% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 24 | 0.79% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 25 | 0.78% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 26 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 27 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 28 | 0.67% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 31 | 0.61% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 527` |
| 32 | 0.58% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 33 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 35 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 36 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 37 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 38 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 40 | 0.46% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 41 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 43 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 45 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 49 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 449` |
| 51 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 52 | 0.29% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.28% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 54 | 0.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 56 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 58 | 0.25% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |
| 59 | 0.24% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 60 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.83% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.49% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 1.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 16 | 1.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 18 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 19 | 0.95% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 21 | 0.91% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.86% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 26 | 0.78% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 28 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 30 | 0.69% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |

GC pauses: 0, total 0 ms


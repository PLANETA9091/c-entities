# JVM hotspots (JFR ExecutionSample)

Samples: 10590

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 2.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 3 | 2.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 6 | 2.02% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 7 | 1.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 14 | 1.34% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 15 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 17 | 1.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 20 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 21 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 23 | 0.93% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 24 | 0.89% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.86% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1271` |
| 26 | 0.85% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 27 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 28 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 29 | 0.78% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 30 | 0.78% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 31 | 0.73% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 32 | 0.72% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 33 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 34 | 0.68% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 35 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 36 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |
| 37 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 38 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 39 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.53% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 41 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 42 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 43 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 45 | 0.46% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 46 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 48 | 0.41% | `java.util.HashMap.getNode(Object) line: 585` |
| 49 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 50 | 0.37% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 51 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 52 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 53 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 54 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 56 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 57 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.32% | `java.lang.ThreadLocal.get() line: 172` |
| 59 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 3.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 3.25% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 2.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 2.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 9 | 2.14% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 10 | 1.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.79% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 1.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 14 | 1.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 1.47% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 16 | 1.45% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 17 | 1.44% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 18 | 1.27% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 19 | 1.25% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 20 | 1.24% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 21 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 22 | 1.06% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 1.03% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 24 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 25 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 26 | 0.98% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1483` |
| 27 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.94% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 187` |
| 29 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.86% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |

GC pauses: 0, total 0 ms


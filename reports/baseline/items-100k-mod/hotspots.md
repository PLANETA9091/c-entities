# JVM hotspots (JFR ExecutionSample)

Samples: 14347

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.62% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.90% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 13 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 102` |
| 14 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.30% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 17 | 1.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 18 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 19 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 20 | 1.02% | `java.lang.Enum.ordinal() line: 128` |
| 21 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 22 | 0.93% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.87% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.81% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 26 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 28 | 0.61% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 29 | 0.59% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 31 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 32 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 34 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.52% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 38 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 39 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 40 | 0.45% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 41 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 42 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 45 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 47 | 0.39% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 49 | 0.36% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.35% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 51 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 56 | 0.29% | `java.util.ArrayDeque.sub(int, int, int) line: 248` |
| 57 | 0.28% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 58 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 59 | 0.26% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 60 | 0.25% | `java.util.EnumMap.unmaskNull(Object) line: 127` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 7.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.92% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 4.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 3.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 3.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.56% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.90% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 37` |
| 15 | 1.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.14% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 17 | 1.11% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 1.01% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 24 | 0.89% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 25 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.81% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 27 | 0.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 28 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |

GC pauses: 0, total 0 ms


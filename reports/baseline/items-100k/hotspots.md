# JVM hotspots (JFR ExecutionSample)

Samples: 16210

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.17% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 19 | 0.96% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.56% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 27 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 29 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 31 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 32 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 33 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 36 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 37 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 38 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 46 | 0.32% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 47 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.31% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 50 | 0.31% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 51 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 52 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.27% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.26% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 60 | 0.25% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 27` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 3.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.30% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.97% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 14 | 1.23% | `java.util.ArrayList.toArray() line: 370` |
| 15 | 1.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 1.05% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 0.99% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 0.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 0.94% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 22 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.86% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.84% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 26 | 0.83% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


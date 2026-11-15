# JVM hotspots (JFR ExecutionSample)

Samples: 16409

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 16 | 0.99% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.65% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.55% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 29 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 31 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 34 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.44% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 37 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 40 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 42 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 46 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 48 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 53 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 25` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 3.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.61% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 2.30% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 1.30% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.13% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 15 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 16 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 1.03% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.99% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 21 | 0.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 23 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.87% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.82% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 30 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |

GC pauses: 0, total 0 ms


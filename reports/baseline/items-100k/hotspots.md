# JVM hotspots (JFR ExecutionSample)

Samples: 15235

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.62% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 1.23% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.15% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.14% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.07% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.81% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 29 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 33 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 35 | 0.44% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.43% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 37 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 40 | 0.40% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 41 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 43 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 45 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 46 | 0.33% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 49 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 50 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 51 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2410` |
| 54 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 56 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 59 | 0.24% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 60 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 6.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.18% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.00% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.23% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.04% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 23 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.73% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 26 | 0.66% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 29 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms


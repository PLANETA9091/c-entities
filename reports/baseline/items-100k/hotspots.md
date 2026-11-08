# JVM hotspots (JFR ExecutionSample)

Samples: 15789

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.93% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.49% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.39% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.34% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 1.10% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.05% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.80% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.70% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 31 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 37 | 0.45% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 42 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 43 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 44 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 45 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 46 | 0.32% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 47 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 48 | 0.29% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 49 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 53 | 0.25% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 54 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.25% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 56 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 60 | 0.23% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.compareAndSet(VarHandle, Object, int, int) line: 178` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.50% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 1.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 1.39% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.06% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.06% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.77% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.76% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 26 | 0.70% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |
| 27 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


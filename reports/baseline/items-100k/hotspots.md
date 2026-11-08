# JVM hotspots (JFR ExecutionSample)

Samples: 13110

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.51% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 8 | 2.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 2.04% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.69% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.27% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.17% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.98% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.78% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 33 | 0.58% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 39 | 0.47% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 41 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 43 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 46 | 0.36% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 47 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 49 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.30% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.28% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 52 | 0.28% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 53 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 54 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 56 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 57 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 58 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 60 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 4.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 6 | 2.74% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 7 | 2.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.52% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 9 | 2.43% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 2.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 2.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.27% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.27% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 1.02% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.91% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.74% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.72% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms


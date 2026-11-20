# JVM hotspots (JFR ExecutionSample)

Samples: 14696

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 2.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.16% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 1.02% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 1.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.97% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 25 | 0.94% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 26 | 0.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.63% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.59% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 35 | 0.50% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 37 | 0.50% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 39 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.40% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 41 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 42 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 44 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 46 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 47 | 0.33% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1414` |
| 48 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 52 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 54 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 55 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.24% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.24% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.23% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 6.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 7 | 2.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.28% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.91% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 1.27% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 16 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.18% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 18 | 1.16% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 1.03% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 22 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.84% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.83% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 25 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.67% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 29 | 0.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 30 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms


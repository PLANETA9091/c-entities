# JVM hotspots (JFR ExecutionSample)

Samples: 16638

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 3.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.28% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 1.05% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 22 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.94% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 28 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 29 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 31 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 33 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 35 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 41 | 0.38% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.33% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 48 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 51 | 0.31% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 54 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 55 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 56 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 60 | 0.25% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 7 | 3.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 10 | 1.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.48% | `java.util.ArrayList.toArray() line: 370` |
| 14 | 1.47% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 15 | 1.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 1.00% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.96% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 21 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.91% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 25 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.77% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

GC pauses: 0, total 0 ms


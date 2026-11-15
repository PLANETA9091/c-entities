# JVM hotspots (JFR ExecutionSample)

Samples: 16573

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 1.24% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 21 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.74% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.63% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 31 | 0.60% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 34 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 37 | 0.39% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 38 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.35% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 40 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 41 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 43 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.32% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.32% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 47 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 53 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 54 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 56 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 59 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.98% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 5 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.61% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 3.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.72% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.69% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.67% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 13 | 1.46% | `java.util.ArrayList.toArray() line: 370` |
| 14 | 1.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 1.08% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.97% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 22 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.94% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 27 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.77% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 29 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

GC pauses: 0, total 0 ms


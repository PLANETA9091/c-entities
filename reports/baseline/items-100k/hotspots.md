# JVM hotspots (JFR ExecutionSample)

Samples: 16334

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 4.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.46% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 1.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 17 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 19 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.98% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.45% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 38 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 41 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 44 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 47 | 0.31% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 48 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.29% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 50 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 51 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 53 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 54 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 55 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 56 | 0.26% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 57 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 58 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 60 | 0.24% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 4.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.44% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 6 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.72% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 14 | 1.38% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 15 | 1.35% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.91% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.74% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 30 | 0.65% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |

GC pauses: 0, total 0 ms


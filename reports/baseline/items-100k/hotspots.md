# JVM hotspots (JFR ExecutionSample)

Samples: 16553

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.14% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.02% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 24 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.91% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.72% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 36 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 41 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.35% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 43 | 0.33% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 44 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 46 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 48 | 0.31% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 51 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 54 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 57 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.25% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 60 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.34% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 5 | 4.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 3.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 1.64% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 14 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.21% | `java.util.ArrayList.toArray() line: 370` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 19 | 1.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.95% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 25 | 0.83% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.73% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 29 | 0.72% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 30 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

GC pauses: 0, total 0 ms


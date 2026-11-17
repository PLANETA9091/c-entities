# JVM hotspots (JFR ExecutionSample)

Samples: 19302

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 2.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.70% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.53% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 14 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 17 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 1.05% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 28 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 32 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 33 | 0.63% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 35 | 0.56% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 37 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 39 | 0.45% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 40 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 41 | 0.43% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 43 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 44 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 47 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 48 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 51 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 52 | 0.31% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.28% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 54 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 55 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 57 | 0.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.24% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 5.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 7 | 2.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.85% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.83% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.67% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 1.67% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 14 | 1.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 18 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 1.01% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.94% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 21 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.83% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 27 | 0.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 28 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.70% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |

GC pauses: 0, total 0 ms


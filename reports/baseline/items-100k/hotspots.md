# JVM hotspots (JFR ExecutionSample)

Samples: 13804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 2.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 8 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.98% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.80% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 11 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.43% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.22% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 1.04% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.93% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.74% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 29 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 31 | 0.59% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 32 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 33 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 35 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 38 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 41 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 44 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.33% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 46 | 0.33% | `java.util.ArrayDeque.size() line: 647` |
| 47 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 49 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 50 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 52 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1254` |
| 54 | 0.27% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 56 | 0.25% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 60 | 0.24% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.82% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 6 | 2.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.22% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 9 | 2.14% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.04% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 13 | 1.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.43% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 15 | 1.22% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 21 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 22 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.89% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 25 | 0.87% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.67% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.62% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 30 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 14741

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 2.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.72% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.15% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 1.10% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 1.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.96% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 26 | 0.86% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.81% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 33 | 0.62% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 34 | 0.55% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 35 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 37 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 38 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 41 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 42 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.37% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 44 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 45 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 46 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 47 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 48 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 50 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 51 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.27% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.26% | `java.lang.invoke.VarHandleInts$FieldInstanceReadWrite.compareAndSet(VarHandle, Object, int, int) line: 178` |
| 55 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 57 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 58 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 60 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 5.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.26% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.55% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 2.12% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 1.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 1.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 22 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.92% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.75% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.66% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


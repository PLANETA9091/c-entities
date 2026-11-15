# JVM hotspots (JFR ExecutionSample)

Samples: 14796

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.81% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.49% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.18% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.16% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 1.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.99% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.95% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.84% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 32 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.59% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 37 | 0.44% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1414` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 39 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 41 | 0.36% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 42 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 44 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.30% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1296` |
| 46 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 47 | 0.29% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 48 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 50 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 51 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 53 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 54 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.25% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 57 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.24% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 5.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 7 | 3.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.01% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 1.37% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.14% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 20 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 21 | 1.02% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 22 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.87% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.80% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.68% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 28 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.64% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.60% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 15878

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.39% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 5 | 2.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 6 | 2.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.37% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 1.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.20% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.97% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.88% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.73% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.71% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 32 | 0.68% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 33 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 36 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 38 | 0.50% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 39 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 41 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 42 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 44 | 0.37% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 45 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 46 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 47 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 48 | 0.28% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1414` |
| 50 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 51 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1254` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 55 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.24% | `java.util.ArrayList$Itr.hasNext() line: 1045` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 59 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 60 | 0.23% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 6.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.38% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 7 | 3.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.02% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 1.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 1.20% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.77% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.73% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 28 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

GC pauses: 0, total 0 ms


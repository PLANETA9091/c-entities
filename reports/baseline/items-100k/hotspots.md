# JVM hotspots (JFR ExecutionSample)

Samples: 13858

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.76% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 17 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 23 | 0.72% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.52% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 31 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 35 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 40 | 0.40% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 41 | 0.40% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 44 | 0.40% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 45 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 53 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 54 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 55 | 0.30% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 57 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 58 | 0.29% | `java.util.ArrayList.clear() line: 735` |
| 59 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 5.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 4.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.93% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.61% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.29% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 1.28% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 18 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.82% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 23 | 0.75% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.74% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.71% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 28 | 0.70% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 30 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |

GC pauses: 0, total 0 ms


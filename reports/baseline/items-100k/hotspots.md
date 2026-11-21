# JVM hotspots (JFR ExecutionSample)

Samples: 17374

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 5 | 3.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 3.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 11 | 1.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 17 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 20 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.93% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 38 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 40 | 0.36% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 41 | 0.36% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.35% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 43 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 47 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.29% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 50 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 53 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 56 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 58 | 0.24% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 7 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 10 | 1.52% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.28% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.28% | `java.util.ArrayList.toArray() line: 370` |
| 17 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 18 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.08% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.98% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 23 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 28 | 0.70% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 29 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 30 | 0.68% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |

GC pauses: 0, total 0 ms


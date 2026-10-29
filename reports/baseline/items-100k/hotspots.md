# JVM hotspots (JFR ExecutionSample)

Samples: 16357

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.12% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 1.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.20% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.80% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 21 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.68% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.56% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 28 | 0.56% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 29 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.53% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 31 | 0.53% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 33 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 40 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 41 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 44 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 47 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 48 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 50 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 53 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 58 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 60 | 0.23% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 4.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 4.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.97% | `java.util.ArrayList.toArray() line: 370` |
| 9 | 1.86% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.97% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 18 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.94% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.84% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.80% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 24 | 0.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 30 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


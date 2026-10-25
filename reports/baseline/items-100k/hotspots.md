# JVM hotspots (JFR ExecutionSample)

Samples: 14008

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.28% | `net.minecraft.util.KeyframeTrackSampler.sample(long) line: 67` |
| 7 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.92% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.19% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.07% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.78% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 23 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.62% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 30 | 0.55% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.55% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 32 | 0.54% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 34 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 40 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.36% | `java.util.ArrayList.clear() line: 735` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 48 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.34% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 52 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 54 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 55 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 57 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 59 | 0.26% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 4 | 4.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 4.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.28% | `net.minecraft.world.timeline.AttributeTrackSampler.applyTimeBased(Object, int) line: 40` |
| 8 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.91% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.76% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.56% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.14% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.02% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 20 | 0.96% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 22 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.85% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 25 | 0.81% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.78% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.78% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 29 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


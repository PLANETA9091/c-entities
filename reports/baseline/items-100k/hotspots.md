# JVM hotspots (JFR ExecutionSample)

Samples: 12881

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 4.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 3.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 17 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.95% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.66% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.54% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 26 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.50% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 32 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 33 | 0.48% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 34 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 40 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 44 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 45 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 48 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 58 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 60 | 0.26% | `java.util.ArrayList.clear() line: 735` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 4.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.61% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.68% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.16% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.15% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 15 | 1.14% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 1.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 1.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.06% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 22 | 0.99% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.93% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 24 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 30 | 0.72% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 14346

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.94% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 12 | 1.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 18 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 20 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 25 | 0.56% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 29 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.48% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 32 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 35 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.42% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.41% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 40 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 46 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 47 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 48 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 50 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 51 | 0.35% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 59 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.57% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.07% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.79% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 13 | 1.29% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.00% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 18 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 19 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.90% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 27 | 0.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 30 | 0.77% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |

GC pauses: 0, total 0 ms


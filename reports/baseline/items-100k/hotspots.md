# JVM hotspots (JFR ExecutionSample)

Samples: 16310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 4.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 3.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 3.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.90% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 11 | 1.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.63% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 19 | 1.22% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.02% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.91% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.72% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.63% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.52% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 38 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 39 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 43 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 44 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.33% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 46 | 0.32% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 47 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 49 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 50 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.31% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 53 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 56 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.27% | `java.util.ArrayDeque.sub(int, int, int) line: 247` |
| 58 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 60 | 0.25% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 5.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.17% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 3.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.87% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 11 | 1.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.46% | `java.util.ArrayList.toArray() line: 370` |
| 15 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.21% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.19% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.96% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.77% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.76% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 27 | 0.74% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.73% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


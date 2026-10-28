# JVM hotspots (JFR ExecutionSample)

Samples: 13310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 5.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.86% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.25% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 17 | 1.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.09% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 25 | 0.92% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 29 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.56% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 32 | 0.54% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 33 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 35 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 38 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 39 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 46 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 47 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.27% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 52 | 0.27% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 53 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 56 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 57 | 0.25% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 58 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 60 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 5.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.86% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.59% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.27% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 1.15% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 0.88% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.87% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.83% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.72% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 28 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |

GC pauses: 0, total 0 ms


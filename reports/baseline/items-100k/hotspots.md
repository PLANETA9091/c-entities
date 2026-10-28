# JVM hotspots (JFR ExecutionSample)

Samples: 13152

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 8.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 5.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.95% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.57% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 16 | 1.33% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.05% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.70% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 25 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 26 | 0.59% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 29 | 0.50% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 30 | 0.46% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 31 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 33 | 0.40% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 34 | 0.39% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 35 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 38 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 42 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.29% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 45 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 46 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 47 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 49 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.27% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 52 | 0.27% | `net.minecraft.world.level.LevelReader.getChunkIfLoadedImmediately(BlockPos) line: 44` |
| 53 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.23% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2406` |
| 58 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2405` |
| 60 | 0.21% | `net.minecraft.util.Mth.floor(float) line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.73% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1854` |
| 2 | 8.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 7.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 7.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 5.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 1.99% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 1.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.33% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.00% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.96% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 19 | 0.96% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 22 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.59% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms


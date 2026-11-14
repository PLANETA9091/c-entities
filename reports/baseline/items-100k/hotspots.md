# JVM hotspots (JFR ExecutionSample)

Samples: 17146

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 1.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.01% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 0.98% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.81% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 27 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 30 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 32 | 0.57% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 33 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 34 | 0.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 35 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 36 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.43% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 40 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 41 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 47 | 0.36% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 48 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 52 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 55 | 0.30% | `java.lang.Double.isFinite(double) line: 1023` |
| 56 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 58 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 59 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.50% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 1.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.21% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.20% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.05% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.97% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.97% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 21 | 0.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 22 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 29 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 30 | 0.71% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |

GC pauses: 0, total 0 ms


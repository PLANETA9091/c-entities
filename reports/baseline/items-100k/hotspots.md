# JVM hotspots (JFR ExecutionSample)

Samples: 17496

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 1.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 1.04% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 17 | 0.98% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.86% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 27 | 0.74% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 32 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.44% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 36 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 37 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 39 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 43 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.33% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 49 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1296` |
| 52 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 54 | 0.29% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1414` |
| 55 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 56 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 59 | 0.27% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.50% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 10 | 1.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 11 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.37% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 15 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.14% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 17 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.03% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 23 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 24 | 0.82% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 27 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 29 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.69% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 17781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 11 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.13% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 1.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 1.05% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.74% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 25 | 0.74% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.62% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 33 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 35 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 38 | 0.41% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 39 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 41 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 43 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 46 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 52 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 53 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 57 | 0.30% | `java.lang.Double.isFinite(double) line: 1023` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 60 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 3.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 3.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.41% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 3.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 1.41% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 12 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 13 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 15 | 1.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.20% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.06% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.01% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 23 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.75% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 27 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.70% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms


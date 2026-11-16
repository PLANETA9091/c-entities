# JVM hotspots (JFR ExecutionSample)

Samples: 13391

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 6 | 2.11% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 7 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 18 | 1.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 1.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 1.12% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 22 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.96% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.93% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 27 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.78% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 29 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 30 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 32 | 0.60% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 33 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 35 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 37 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 40 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.39% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 43 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.34% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 47 | 0.34% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 49 | 0.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 50 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 51 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 54 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 56 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 59 | 0.28% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 160` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 4.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 2.96% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 6 | 2.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.44% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 8 | 2.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 9 | 2.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.94% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 1.61% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 15 | 1.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 1.08% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 22 | 0.96% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.90% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.76% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 27 | 0.72% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 28 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.isInRain() line: 2170` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 13768

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 5.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.32% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 6 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 1.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.49% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.87% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 25 | 0.81% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 26 | 0.79% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.73% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.68% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.50% | `net.minecraft.world.entity.Entity.getXRot() line: 5297` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.46% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 37 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 39 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 40 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 46 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 47 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 51 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 298` |
| 58 | 0.31% | `java.util.HashMap.getNode(Object) line: 586` |
| 59 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 4.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 5 | 3.42% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 6 | 3.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 2.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 10 | 2.03% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 15 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 17 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 20 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 23 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.78% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 25 | 0.77% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 26 | 0.73% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 27 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.68% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms


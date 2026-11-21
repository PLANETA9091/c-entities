# JVM hotspots (JFR ExecutionSample)

Samples: 14595

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.93% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 5 | 2.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.46% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 8 | 2.00% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 1.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 10 | 1.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 16 | 1.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.08% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 1.05% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.98% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.96% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.89% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.79% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 29 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 42 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 44 | 0.40% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 45 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 49 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 54 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 57 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 58 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 60 | 0.27% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 4.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.02% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 6 | 2.76% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.45% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 9 | 2.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 2.14% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 2.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.17% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 1.03% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 19 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.92% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 23 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.84% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 25 | 0.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.79% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.69% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 29 | 0.68% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms


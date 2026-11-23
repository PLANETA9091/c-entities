# JVM hotspots (JFR ExecutionSample)

Samples: 14879

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 3.01% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 5 | 2.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 2.08% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 2.04% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 16 | 1.22% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 17 | 1.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 18 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 1.06% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 22 | 1.04% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.82% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 30 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 31 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.40% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 41 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 44 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.36% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 48 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 51 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 56 | 0.29% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 59 | 0.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 3 | 4.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.13% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 6 | 2.82% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 2.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 19 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.89% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 24 | 0.83% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.81% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.78% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 27 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |

GC pauses: 0, total 0 ms


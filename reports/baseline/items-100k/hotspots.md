# JVM hotspots (JFR ExecutionSample)

Samples: 15060

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.50% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 8 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.78% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 10 | 1.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.04% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 1.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.88% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 29 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 32 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 34 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 35 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.49% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 38 | 0.44% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 39 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 40 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 48 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.33% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 53 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 54 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 57 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 60 | 0.27% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 4.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 7 | 2.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.60% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 1.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 15 | 1.26% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.07% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.07% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 1.00% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 21 | 0.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 22 | 0.88% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 25 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 30 | 0.68% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms


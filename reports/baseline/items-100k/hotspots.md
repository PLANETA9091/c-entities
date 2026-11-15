# JVM hotspots (JFR ExecutionSample)

Samples: 16882

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 10 | 1.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.08% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 1.01% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 0.78% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 25 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 27 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 30 | 0.62% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 34 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 35 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 38 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 39 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 43 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 45 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 48 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.33% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 55 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.30% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 5.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.42% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.60% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 11 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.17% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.11% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.07% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1412` |
| 16 | 1.06% | `java.util.ArrayList.toArray() line: 370` |
| 17 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 25 | 0.84% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 26 | 0.82% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.78% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 29 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 30 | 0.75% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms


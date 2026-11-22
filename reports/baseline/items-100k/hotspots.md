# JVM hotspots (JFR ExecutionSample)

Samples: 17579

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 7 | 2.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 1.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.84% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.18% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 16 | 1.11% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 21 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.68% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 28 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.59% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.57% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.51% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 38 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 41 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 42 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 46 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 47 | 0.31% | `java.util.HashMap.getNode(Object) line: 586` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 49 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 51 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 54 | 0.29% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 55 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.27% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 57 | 0.27% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 58 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 4.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 4.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 9 | 1.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 10 | 1.68% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.52% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.12% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 18 | 1.10% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 19 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 1.02% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 1.01% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 22 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.76% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.75% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


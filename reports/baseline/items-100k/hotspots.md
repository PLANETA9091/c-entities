# JVM hotspots (JFR ExecutionSample)

Samples: 14534

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 3.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.71% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 0.99% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 17 | 0.96% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 19 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 26 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 27 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 31 | 0.54% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 33 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 35 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 38 | 0.44% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 39 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 41 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 42 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 46 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.36% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 50 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 53 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 54 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 57 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 58 | 0.28% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 59 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 4.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 4.25% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.64% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.60% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.11% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 14 | 1.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 17 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.02% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 19 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 22 | 0.95% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 23 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.92% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 25 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 27 | 0.84% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 28 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


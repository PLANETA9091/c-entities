# JVM hotspots (JFR ExecutionSample)

Samples: 13026

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 5.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.76% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 5 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 1.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.77% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.64% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 11 | 1.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.24% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 18 | 1.14% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2410` |
| 25 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.55% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 27 | 0.55% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 31 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 33 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 35 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 37 | 0.44% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 38 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.41% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 43 | 0.38% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 44 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.38% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 306` |
| 46 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 47 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 49 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.32% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.29% | `net.minecraft.server.MinecraftServer.moonrise$executeMidTickTasks() line: 462` |
| 57 | 0.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 60 | 0.27% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 2 | 6.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 2.89% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.66% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.64% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 13 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.60% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 17 | 1.14% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 1.01% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 22 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.86% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 24 | 0.84% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 25 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 28 | 0.74% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 29 | 0.72% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


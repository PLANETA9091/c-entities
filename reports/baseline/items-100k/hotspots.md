# JVM hotspots (JFR ExecutionSample)

Samples: 13056

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 3.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 11 | 1.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 15 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 17 | 1.06% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.77% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 25 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.66% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 29 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.55% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 32 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.46% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 45 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 47 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.36% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 52 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 53 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 56 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 235` |
| 58 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 60 | 0.26% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 3.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.82% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.08% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.58% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.26% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 15 | 1.23% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 16 | 1.19% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 19 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 1.06% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 21 | 1.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 1.03% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.99% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 24 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 25 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 27 | 0.87% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 28 | 0.86% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.78% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


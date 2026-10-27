# JVM hotspots (JFR ExecutionSample)

Samples: 12776

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 6 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 16 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 20 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.70% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 22 | 0.66% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 23 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 26 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 27 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 29 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.55% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 31 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.47% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.41% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 44 | 0.37% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 45 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 47 | 0.34% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 51 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 53 | 0.31% | `java.util.HashMap.getNode(Object) line: 588` |
| 54 | 0.31% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 55 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 57 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 58 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.isCollidingWithBorder(WorldBorder, double, double, double, double) line: 1871` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 3.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.71% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.50% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.13% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 11 | 1.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.90% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 15 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 16 | 1.18% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 17 | 1.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.09% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 1.06% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 22 | 0.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.86% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 26 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.77% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.73% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |

GC pauses: 0, total 0 ms


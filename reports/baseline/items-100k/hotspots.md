# JVM hotspots (JFR ExecutionSample)

Samples: 13911

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 12 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 18 | 1.06% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 21 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.93% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 26 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 27 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 32 | 0.60% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 33 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 35 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 36 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.36% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 48 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 49 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 160` |
| 51 | 0.31% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 52 | 0.29% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 53 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 54 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 56 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 57 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 58 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 59 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 60 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 2.98% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 7 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 10 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.51% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 13 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 1.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.27% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 16 | 1.24% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 17 | 1.19% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 18 | 1.11% | `java.util.ArrayList.toArray() line: 370` |
| 19 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1895` |
| 22 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 25 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.72% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 27 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.68% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |

GC pauses: 0, total 0 ms


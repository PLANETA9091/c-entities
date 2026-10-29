# JVM hotspots (JFR ExecutionSample)

Samples: 14918

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.93% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 13 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 1.62% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.97% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 28 | 0.62% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.50% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 30 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 31 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.47% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 33 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 34 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.42% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 37 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.37% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 40 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 42 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 45 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 48 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 50 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.29% | `java.util.ArrayDeque.sub(int, int, int) line: 247` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.28% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 56 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 59 | 0.25% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 60 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.68% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.19% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.48% | `java.util.ArrayList.toArray() line: 370` |
| 15 | 1.19% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 16 | 1.15% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.72% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 29 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 30 | 0.62% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 12959

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.39% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 1.20% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 21 | 1.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 1.10% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 1.02% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.58% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 32 | 0.52% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.52% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 35 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 37 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 42 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 44 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 45 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.33% | `java.util.ArrayDeque.sub(int, int, int) line: 247` |
| 49 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 50 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 51 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 52 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 54 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 57 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.25% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.25% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 60 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.41% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.48% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.14% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.42% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 16 | 1.30% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.19% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.18% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 1.17% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 1.04% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 1.02% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 23 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.59% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


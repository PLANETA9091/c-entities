# JVM hotspots (JFR ExecutionSample)

Samples: 12436

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.02% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 16 | 1.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.29% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 1.19% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 21 | 1.04% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 1.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 24 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.88% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 28 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 29 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.68% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 32 | 0.47% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 33 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 36 | 0.46% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 41 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 45 | 0.32% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 46 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 48 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 49 | 0.30% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 54 | 0.29% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 55 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 57 | 0.27% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 58 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 60 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 7.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.21% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 2.09% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.08% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.54% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 15 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 0.99% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 20 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 27 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 30 | 0.57% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |

GC pauses: 0, total 0 ms


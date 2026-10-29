# JVM hotspots (JFR ExecutionSample)

Samples: 11178

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.20% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 4 | 4.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 2.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.87% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.38% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 18 | 1.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.25% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 20 | 1.20% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 1.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 27 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.69% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 30 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.55% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 36 | 0.39% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 37 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 39 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 40 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 41 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 887` |
| 43 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 44 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 46 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 47 | 0.32% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 48 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 49 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 50 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 52 | 0.30% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 55 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 56 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.getXRot() line: 5292` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 160` |
| 59 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.54% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 2.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 8 | 2.50% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.12% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.51% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 1.49% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 1.32% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.11% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.84% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.73% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 29 | 0.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |

GC pauses: 0, total 0 ms


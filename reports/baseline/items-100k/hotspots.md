# JVM hotspots (JFR ExecutionSample)

Samples: 12307

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 3.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 10 | 1.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 14 | 1.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 16 | 1.41% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 19 | 1.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 20 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 21 | 1.15% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.03% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 24 | 0.94% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 25 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 30 | 0.81% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 31 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 33 | 0.59% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 35 | 0.46% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 40 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 46 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 49 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 50 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.28% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 52 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 53 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 54 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 56 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 7.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 7.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 5.44% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 7 | 2.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.51% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 9 | 2.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.91% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 14 | 1.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.25% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 18 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.03% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.98% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 21 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.isInRain() line: 2164` |
| 27 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 29 | 0.59% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 491` |
| 30 | 0.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |

GC pauses: 0, total 0 ms


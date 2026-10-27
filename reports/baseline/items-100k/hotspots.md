# JVM hotspots (JFR ExecutionSample)

Samples: 11945

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.80% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 13 | 1.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 16 | 1.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 1.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 0.99% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.98% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.64% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 31 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.59% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 34 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 36 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 39 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 40 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 46 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 48 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 49 | 0.35% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 50 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.33% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.28% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 58 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 6.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 4.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.22% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.97% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 11 | 1.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 12 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.71% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 15 | 1.63% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 1.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 1.31% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 19 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 1.03% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 22 | 0.87% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 27 | 0.64% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 28 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |

GC pauses: 0, total 0 ms


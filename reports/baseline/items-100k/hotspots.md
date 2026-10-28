# JVM hotspots (JFR ExecutionSample)

Samples: 12535

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.70% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 7 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 2.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.79% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.76% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.75% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.64% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.52% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.37% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.20% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 1.05% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 1.01% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 28 | 0.77% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 29 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 30 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.69% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.61% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 34 | 0.56% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 36 | 0.46% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 37 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.38% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 39 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 41 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 45 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.32% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 51 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.30% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 53 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 55 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 56 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.26% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 6.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 5.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.98% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 7 | 2.35% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.26% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 2.02% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.75% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 14 | 1.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.52% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 17 | 1.32% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 22 | 1.01% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 26 | 0.74% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 27 | 0.71% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |

GC pauses: 0, total 0 ms


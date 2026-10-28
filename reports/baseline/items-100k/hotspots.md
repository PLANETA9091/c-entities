# JVM hotspots (JFR ExecutionSample)

Samples: 11519

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 7 | 2.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 1.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.88% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.81% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 15 | 1.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 16 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 1.42% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 20 | 1.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 1.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 23 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 1.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.76% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 31 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 37 | 0.44% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 40 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.37% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 45 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 47 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.30% | `java.util.ArrayDeque.sub(int, int, int) line: 247` |
| 53 | 0.29% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 54 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 55 | 0.29% | `net.minecraft.world.entity.Entity.getXRot() line: 5292` |
| 56 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 57 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 59 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 60 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.95% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 3.16% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 3.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 2.15% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.02% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.42% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.30% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.27% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 1.02% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 1.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 25 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 27 | 0.64% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.63% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


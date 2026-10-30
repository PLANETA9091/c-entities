# JVM hotspots (JFR ExecutionSample)

Samples: 13543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 8.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.59% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.96% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 12 | 1.74% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.18% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 1.00% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 25 | 0.58% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 26 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 27 | 0.56% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 31 | 0.47% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 32 | 0.44% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 33 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 35 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.41% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 38 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 41 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.37% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 43 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 46 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 47 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 48 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 51 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 52 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.26% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 54 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.25% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 56 | 0.24% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 57 | 0.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2406` |
| 58 | 0.24% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 59 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.23% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.56% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1854` |
| 2 | 8.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 7.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 6.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 5.04% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.97% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.85% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 13 | 1.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 14 | 1.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.19% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 19 | 0.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 0.83% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 22 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 23 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.59% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 491` |
| 27 | 0.56% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 30 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 12470

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 2 | 7.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 4.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 4.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 6 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.22% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 8 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.66% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 14 | 1.33% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 17 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.91% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.87% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.75% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 24 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.57% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 32 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 35 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 36 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 37 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 38 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 39 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.34% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 41 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 42 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 44 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 46 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.28% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 50 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 51 | 0.27% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 53 | 0.27% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 54 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 57 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 58 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.22% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 60 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1854` |
| 2 | 7.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 4.42% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.12% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 1.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.76% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 12 | 1.49% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 14 | 1.33% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 17 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.91% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 21 | 0.87% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.85% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.57% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 491` |
| 30 | 0.55% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |

GC pauses: 0, total 0 ms


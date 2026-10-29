# JVM hotspots (JFR ExecutionSample)

Samples: 13256

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.06% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.77% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 14 | 1.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 16 | 1.49% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.40% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 18 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 1.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 1.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 1.18% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 1.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.98% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.96% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 27 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.60% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.57% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 36 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 38 | 0.48% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 39 | 0.48% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 41 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 45 | 0.34% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 47 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.32% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 49 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 51 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 54 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 58 | 0.26% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 59 | 0.26% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 7.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.08% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 2.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 2.07% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.90% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 14 | 1.55% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.46% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.40% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 18 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 1.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 21 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.90% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 28 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.isInRain() line: 2165` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 12616

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.97% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.96% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.81% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 14 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.18% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.01% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 1.01% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 23 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 0.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 31 | 0.53% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 35 | 0.48% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 36 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 43 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.33% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 45 | 0.32% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 46 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 47 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 48 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.25% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 60 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.25% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.07% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.39% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.03% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 12 | 2.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.52% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 16 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.22% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.17% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.90% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.isInRain() line: 2170` |
| 26 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 28 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.53% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 30 | 0.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |

GC pauses: 0, total 0 ms


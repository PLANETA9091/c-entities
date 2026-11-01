# JVM hotspots (JFR ExecutionSample)

Samples: 12979

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 5.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.09% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.79% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.82% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 31 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 32 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 33 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 37 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 39 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 43 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 48 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 49 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 51 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.27% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 54 | 0.26% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2410` |
| 56 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.25% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 58 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 59 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 5.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 2.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 15 | 1.26% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 1.18% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 20 | 0.76% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.58% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 26 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.isInRain() line: 2170` |
| 28 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.52% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 30 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms


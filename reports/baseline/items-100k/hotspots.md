# JVM hotspots (JFR ExecutionSample)

Samples: 13722

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 4.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.69% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 1.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 21 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.04% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.99% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 30 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.66% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.44% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 37 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.42% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 40 | 0.41% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 43 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 45 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 47 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 50 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 53 | 0.29% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 54 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 55 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 57 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.26% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 59 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.24% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.47% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.22% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.03% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.65% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.22% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.21% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 0.96% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.88% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.86% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.70% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 29 | 0.65% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


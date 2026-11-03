# JVM hotspots (JFR ExecutionSample)

Samples: 13781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 2.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.86% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.84% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 1.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.11% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.99% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.86% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.62% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 30 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 33 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 36 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 41 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 42 | 0.35% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 43 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 44 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 47 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 52 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 53 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.26% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 191` |
| 56 | 0.25% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 58 | 0.25% | `net.minecraft.core.Vec3i.distToCenterSqr(double, double, double) line: 209` |
| 59 | 0.24% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 60 | 0.24% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 5.36% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.37% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 2.31% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 1.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.67% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.17% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 19 | 0.97% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 21 | 0.75% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 23 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 26 | 0.66% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.64% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 28 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.58% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |

GC pauses: 0, total 0 ms


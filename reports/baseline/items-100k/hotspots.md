# JVM hotspots (JFR ExecutionSample)

Samples: 12303

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.43% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.98% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.87% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 16 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.29% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 18 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 19 | 1.19% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 1.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.54% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.54% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.48% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 38 | 0.42% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 39 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.39% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 41 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 44 | 0.32% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 45 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.28% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 50 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 51 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 52 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 54 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 56 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.24% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 59 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.08% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 10 | 1.99% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.48% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 14 | 1.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.27% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 19 | 1.17% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.89% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.67% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 25 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 27 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |

GC pauses: 0, total 0 ms


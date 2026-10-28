# JVM hotspots (JFR ExecutionSample)

Samples: 13113

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 5.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.72% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.56% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 1.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 22 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 24 | 1.00% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 25 | 0.85% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.79% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 30 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.57% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 35 | 0.50% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 39 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 43 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.39% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 45 | 0.38% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 46 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.31% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 50 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 51 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 52 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.25% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.25% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 58 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.23% | `net.minecraft.server.MinecraftServer.moonrise$executeMidTickTasks() line: 462` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 5.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 9 | 2.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.10% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 11 | 2.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.85% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.56% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.26% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 19 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.83% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.69% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 27 | 0.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 28 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.60% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 30 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


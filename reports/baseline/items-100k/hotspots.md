# JVM hotspots (JFR ExecutionSample)

Samples: 14201

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 5.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.99% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.86% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 16 | 1.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.08% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 1.04% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 1.00% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.82% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.66% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.54% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 32 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.49% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 38 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.42% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 40 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 41 | 0.37% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 42 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 44 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 45 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 52 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 55 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 56 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 57 | 0.24% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 59 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.23% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.52% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 2.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 2.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.11% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.02% | `java.util.ArrayList.toArray() line: 370` |
| 19 | 1.01% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 21 | 0.82% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 23 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.72% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.66% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.65% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 28 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.isInRain() line: 2170` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


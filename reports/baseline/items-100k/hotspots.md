# JVM hotspots (JFR ExecutionSample)

Samples: 12223

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.54% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.99% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.46% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 1.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 19 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 1.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.88% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 28 | 0.70% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 29 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 36 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 39 | 0.44% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 40 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 46 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.34% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 48 | 0.31% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 49 | 0.30% | `net.minecraft.world.level.LevelReader.getChunkIfLoadedImmediately(BlockPos) line: 44` |
| 50 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.commonTick() line: 887` |
| 52 | 0.28% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 53 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.27% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 55 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 56 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 60 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 6.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 5.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.85% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 7 | 2.68% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.52% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.99% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.51% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 16 | 1.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 17 | 1.42% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 18 | 1.36% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.19% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 1.00% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.67% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.56% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 30 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms


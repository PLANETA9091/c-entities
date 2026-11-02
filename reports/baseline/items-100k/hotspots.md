# JVM hotspots (JFR ExecutionSample)

Samples: 10914

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 4.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.90% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 5 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 2.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 2.20% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.55% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.50% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 15 | 1.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 16 | 1.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 1.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 19 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 1.12% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 25 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.92% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.53% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 35 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 42 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 44 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 45 | 0.41% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 46 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.getYRot() line: 5280` |
| 48 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 49 | 0.37% | `net.minecraft.world.item.ItemStack.getComponents() line: 235` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 53 | 0.34% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 54 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 58 | 0.29% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 6.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 5 | 4.18% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 2.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 7 | 2.74% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 2.68% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 2.14% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 13 | 1.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.33% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 17 | 1.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.30% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 19 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.15% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 0.97% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 24 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.71% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 27 | 0.71% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 11` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 12837

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.50% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 1.44% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 18 | 1.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 1.27% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 20 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 1.15% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 1.04% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.81% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.64% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.59% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 37 | 0.48% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 39 | 0.44% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 40 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 45 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList$1.next() line: 853` |
| 47 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 48 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 49 | 0.33% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 53 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.28% | `java.util.ArrayList.grow(int) line: 238` |
| 55 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.commonTick() line: 887` |
| 59 | 0.26% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 60 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 9 | 2.41% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 2.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.57% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 14 | 1.54% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 15 | 1.49% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 16 | 1.47% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 17 | 1.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 1.05% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 21 | 1.04% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.94% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.69% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 26 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.64% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


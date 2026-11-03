# JVM hotspots (JFR ExecutionSample)

Samples: 11636

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 2.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 2.05% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 12 | 2.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.92% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.61% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.50% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 1.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 1.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 1.19% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 1.13% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 1.12% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 24 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.96% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.58% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 33 | 0.54% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 34 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 38 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 40 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 43 | 0.36% | `net.minecraft.world.entity.Entity.commonTick() line: 887` |
| 44 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 46 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 48 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 51 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 53 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.30% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 55 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 57 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 59 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 6.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 6.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.17% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.44% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 9 | 2.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.25% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 2.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.50% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.44% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 17 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.28% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 20 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 1.01% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 22 | 0.83% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.isInRain() line: 2170` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

GC pauses: 0, total 0 ms


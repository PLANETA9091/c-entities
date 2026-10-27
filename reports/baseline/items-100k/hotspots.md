# JVM hotspots (JFR ExecutionSample)

Samples: 13835

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 3.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 5 | 2.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.98% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.93% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.25% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.99% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.62% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 33 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 39 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 42 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.35% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 44 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.34% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 46 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.32% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 49 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 50 | 0.30% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 51 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 53 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 55 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 56 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 58 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.25% | `java.util.ArrayList.clear() line: 735` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.04% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 7 | 2.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 2.16% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.99% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.81% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 16 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 18 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.86% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.69% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 29 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


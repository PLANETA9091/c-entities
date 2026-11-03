# JVM hotspots (JFR ExecutionSample)

Samples: 16639

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 4.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.02% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.78% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 15 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.93% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.75% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.72% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.61% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.52% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 35 | 0.52% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 37 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 42 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 45 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 49 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 51 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 54 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 55 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 58 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 60 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.71% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.65% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.57% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.93% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.90% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 21 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.89% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 23 | 0.84% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 24 | 0.83% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.80% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 27 | 0.75% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |

GC pauses: 0, total 0 ms


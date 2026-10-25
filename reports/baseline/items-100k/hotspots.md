# JVM hotspots (JFR ExecutionSample)

Samples: 14669

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 4.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 8 | 2.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.50% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.07% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 18 | 1.00% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 0.85% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 0.84% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 31 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 32 | 0.54% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.52% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 35 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 38 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 40 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 42 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 45 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 46 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 49 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 52 | 0.29% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 53 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 521` |
| 54 | 0.29% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 55 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 347` |
| 58 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 59 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 5.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 3.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.15% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 9 | 1.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 10 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.53% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.50% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.06% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 1.06% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 19 | 1.00% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.89% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 24 | 0.88% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.87% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.85% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 28 | 0.84% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 29 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.73% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |

GC pauses: 0, total 0 ms


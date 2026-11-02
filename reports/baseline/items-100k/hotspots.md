# JVM hotspots (JFR ExecutionSample)

Samples: 16681

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 3.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.29% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 18 | 1.08% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.95% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.77% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 22 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.64% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.62% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.61% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 32 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 34 | 0.54% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.49% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 39 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.37% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 44 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 45 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 46 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 49 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 52 | 0.29% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.26% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 56 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 57 | 0.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 58 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.23% | `net.minecraft.util.SimpleBitStorage.get(int) line: 347` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.95% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 4.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.56% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.79% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.77% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 12 | 1.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.29% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.94% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 19 | 0.92% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 0.88% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.81% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 24 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.77% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.75% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 28 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 30 | 0.70% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms


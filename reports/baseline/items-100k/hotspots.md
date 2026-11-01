# JVM hotspots (JFR ExecutionSample)

Samples: 16752

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.93% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 16 | 1.22% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.15% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.72% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.61% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.54% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.51% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 42 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 44 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 48 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 56 | 0.26% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 57 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 58 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.23% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 3.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.82% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.95% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.79% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.72% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.21% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.89% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 20 | 0.88% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.79% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 22 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 16097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.48% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 1.06% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.78% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 22 | 0.76% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 23 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.71% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 25 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 26 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 31 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 42 | 0.42% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 43 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 47 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 49 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 56 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.26% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 59 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 60 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.62% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.56% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 10 | 1.70% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.48% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.98% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 19 | 0.94% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 20 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.82% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.76% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 26 | 0.73% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 27 | 0.71% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |

GC pauses: 0, total 0 ms


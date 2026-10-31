# JVM hotspots (JFR ExecutionSample)

Samples: 14535

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 2 | 7.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.87% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 0.85% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 17 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.69% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 22 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 25 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.52% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 31 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 33 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 35 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 36 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 37 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 38 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 41 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 45 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 47 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 51 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 53 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 56 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 347` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 58 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 59 | 0.25% | `java.util.ArrayList.clear() line: 735` |
| 60 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 2 | 7.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 4.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.34% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.54% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.86% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 11 | 1.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 14 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 16 | 1.01% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 17 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.95% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.93% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |
| 23 | 0.80% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.69% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.69% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 29 | 0.69% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms


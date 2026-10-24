# JVM hotspots (JFR ExecutionSample)

Samples: 13584

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 5 | 2.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 8 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.94% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.10% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 15 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 19 | 0.81% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 21 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.75% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 23 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 24 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 27 | 0.65% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 32 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.41% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 40 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 41 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 44 | 0.36% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 46 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 48 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.34% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 51 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.32% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 54 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 56 | 0.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 58 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 60 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 3.47% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 7 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.91% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 10 | 1.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.80% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 13 | 1.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 15 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 16 | 1.07% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 17 | 1.07% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.02% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 19 | 0.99% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 22 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 24 | 0.81% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 25 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.72% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |

GC pauses: 0, total 0 ms


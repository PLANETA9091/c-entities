# JVM hotspots (JFR ExecutionSample)

Samples: 14981

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.22% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 9 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 15 | 1.18% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.81% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 24 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 25 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 29 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 30 | 0.59% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 34 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 35 | 0.52% | `java.util.Optional.isPresent() line: 154` |
| 36 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 41 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.39% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 46 | 0.39% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 47 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 51 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 52 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 53 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 58 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.49% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 9 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.53% | `net.minecraft.util.KeyframeTrackSampler.getSegmentAt(long) line: 71` |
| 12 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.38% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 15 | 1.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.17% | `java.util.ArrayList.toArray() line: 370` |
| 17 | 1.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 1.01% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.00% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 21 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.96% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 27 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.69% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 12460

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 4.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.65% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 4 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 1.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 1.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 11 | 1.55% | `net.minecraft.util.KeyframeTrackSampler.sample(long) line: 67` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 17 | 1.24% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 19 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 22 | 0.98% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 23 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 24 | 0.80% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 26 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 30 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.53% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 37 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 41 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 42 | 0.44% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 43 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 44 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 47 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 49 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.getXRot() line: 5291` |
| 51 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.36% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 56 | 0.34% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 57 | 0.34% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 306` |
| 58 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 59 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 4.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.80% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 6 | 2.86% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 7 | 1.87% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 8 | 1.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 10 | 1.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 11 | 1.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.55% | `net.minecraft.world.timeline.AttributeTrackSampler.applyTimeBased(Object, int) line: 40` |
| 14 | 1.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 15 | 1.38% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.38% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 17 | 1.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.21% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 19 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 20 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 21 | 1.02% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 22 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.97% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 24 | 0.89% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 25 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1895` |
| 29 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.78% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1690` |

GC pauses: 0, total 0 ms


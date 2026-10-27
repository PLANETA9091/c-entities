# JVM hotspots (JFR ExecutionSample)

Samples: 14430

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 4 | 3.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.82% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 10 | 1.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 11 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.52% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 15 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 1.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 18 | 1.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 20 | 1.13% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 21 | 1.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 1.00% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.98% | `java.util.Optional.isPresent() line: 154` |
| 25 | 0.93% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.80% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 28 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.73% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 30 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 31 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 32 | 0.67% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 33 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 34 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.42% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 36 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 39 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 40 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 42 | 0.36% | `net.minecraft.util.Mth.floor(float) line: 63` |
| 43 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 45 | 0.34% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 46 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.33% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 48 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 52 | 0.30% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 53 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 56 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 57 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 58 | 0.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 59 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 60 | 0.26% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 6.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 2.81% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 7 | 2.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.22% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 2.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 2.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 1.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 12 | 1.84% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 14 | 1.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 16 | 1.47% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 17 | 1.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 18 | 1.21% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.16% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 20 | 1.13% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 21 | 0.99% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 25 | 0.81% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 26 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


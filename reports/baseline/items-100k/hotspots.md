# JVM hotspots (JFR ExecutionSample)

Samples: 12816

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 3.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 2.48% | `net.minecraft.world.entity.Entity.commonTick() line: 876` |
| 6 | 2.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 16 | 1.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 17 | 1.31% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 20 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 22 | 1.05% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 23 | 0.94% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 24 | 0.89% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 26 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1842` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 30 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 31 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 32 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 33 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 34 | 0.60% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1844` |
| 36 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 37 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 39 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.43% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 41 | 0.42% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 42 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 44 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 45 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 48 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 49 | 0.40% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 50 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 54 | 0.36% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1413` |
| 55 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 60 | 0.31% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.09% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 6 | 2.66% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 7 | 2.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 8 | 1.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.61% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 13 | 1.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 14 | 1.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 15 | 1.33% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.27% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 17 | 1.26% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 18 | 1.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 1.13% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 1.13% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 21 | 1.11% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1562` |
| 22 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 1.06% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 26 | 0.94% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1895` |
| 27 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 28 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1127` |
| 30 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


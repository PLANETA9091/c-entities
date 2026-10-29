# JVM hotspots (JFR ExecutionSample)

Samples: 14509

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 5 | 2.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.14% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.14% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.01% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 0.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 23 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.73% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.70% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 28 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.58% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.57% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 37 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 41 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.44% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 45 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.40% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.39% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 48 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 51 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 52 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 53 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.32% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 60 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 3.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 1.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 9 | 1.95% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.93% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 11 | 1.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.39% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.33% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 1.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.03% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.00% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 19 | 0.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.93% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.85% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 24 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 25 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.75% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 29 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 15310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 17 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.96% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 0.90% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 22 | 0.75% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.61% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 32 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.57% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.54% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 37 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 39 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 41 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 42 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.37% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 50 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 57 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 58 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 60 | 0.25% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.56% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.89% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.82% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 12 | 1.49% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.20% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 15 | 1.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.05% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.97% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 21 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.74% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 28 | 0.73% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 29 | 0.69% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


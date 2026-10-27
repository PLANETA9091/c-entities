# JVM hotspots (JFR ExecutionSample)

Samples: 14031

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.21% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 2.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 2.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.15% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.00% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 0.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.77% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 22 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.69% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 30 | 0.56% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 32 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.51% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 34 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.43% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 39 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 42 | 0.37% | `net.minecraft.world.entity.Entity.getVehicle() line: 4934` |
| 43 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 44 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 45 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 47 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 49 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 51 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 54 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 55 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 57 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 60 | 0.26% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.41% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.26% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 9 | 2.22% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.82% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.98% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 17 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 18 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.89% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 20 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 22 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 28 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.69% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.67% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |

GC pauses: 0, total 0 ms


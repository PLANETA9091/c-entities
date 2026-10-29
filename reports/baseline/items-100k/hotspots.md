# JVM hotspots (JFR ExecutionSample)

Samples: 15459

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.75% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.31% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.09% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.84% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.82% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 28 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 29 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 30 | 0.65% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 32 | 0.58% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 36 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.49% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 41 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 44 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.43% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 49 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.34% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 53 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 54 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 58 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.26% | `java.util.ArrayList.grow(int) line: 238` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 4.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.31% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.41% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 10 | 1.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.75% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 13 | 1.46% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.41% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.31% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 16 | 1.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.11% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 1.02% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 20 | 0.96% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 23 | 0.83% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.82% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 25 | 0.81% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 27 | 0.78% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.75% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


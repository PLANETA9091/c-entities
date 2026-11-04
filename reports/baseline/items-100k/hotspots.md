# JVM hotspots (JFR ExecutionSample)

Samples: 15498

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.87% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 16 | 1.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.09% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 23 | 0.78% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.75% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 28 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.63% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.56% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 34 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 40 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 41 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 46 | 0.34% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 48 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 49 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 52 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 54 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 56 | 0.27% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 184` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 58 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 59 | 0.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 4.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.58% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.77% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.67% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 11 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.61% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.28% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.01% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 18 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 20 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.86% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 22 | 0.86% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.84% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 26 | 0.75% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


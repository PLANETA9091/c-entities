# JVM hotspots (JFR ExecutionSample)

Samples: 16472

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 8 | 2.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 9 | 1.67% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.17% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 1.09% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.82% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.78% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 26 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.61% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.58% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 33 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 40 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 41 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 43 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 45 | 0.38% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 47 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 48 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 50 | 0.30% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 52 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 53 | 0.29% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 56 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 57 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 59 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 60 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 6 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.52% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.08% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 9 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.74% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.66% | `java.util.ArrayList.toArray() line: 370` |
| 13 | 1.46% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.12% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 1.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.04% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 22 | 0.93% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.92% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 24 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.78% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 28 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 30 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms


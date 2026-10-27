# JVM hotspots (JFR ExecutionSample)

Samples: 13967

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 13 | 1.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.45% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 17 | 1.16% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.02% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.83% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 21 | 0.82% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.72% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.62% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 28 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 31 | 0.54% | `java.util.Optional.get() line: 142` |
| 32 | 0.54% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 34 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 37 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 41 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.39% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 43 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 52 | 0.31% | `java.util.ArrayDeque.sub(int, int, int) line: 247` |
| 53 | 0.30% | `java.util.ArrayList.clear() line: 735` |
| 54 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 521` |
| 56 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 57 | 0.27% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 58 | 0.27% | `net.minecraft.core.BlockPos.containing(double, double, double) line: 91` |
| 59 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 5.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 4.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 6 | 3.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.48% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.11% | `java.util.ArrayList.toArray() line: 370` |
| 9 | 1.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 13 | 1.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.17% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 17 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 1.00% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 19 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 20 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.84% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 24 | 0.84% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 25 | 0.83% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 26 | 0.81% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 27 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

GC pauses: 0, total 0 ms


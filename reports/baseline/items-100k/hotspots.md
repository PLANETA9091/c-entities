# JVM hotspots (JFR ExecutionSample)

Samples: 15597

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 1.61% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 14 | 1.21% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 0.97% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 18 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.96% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 21 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 25 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 30 | 0.61% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.54% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.54% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 37 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.49% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 40 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.40% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 45 | 0.39% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 48 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 51 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 52 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 73` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 60 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 5 | 4.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.80% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.88% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 10 | 1.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.60% | `java.util.ArrayList.toArray() line: 370` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 13 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 14 | 1.35% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 16 | 1.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 0.97% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.82% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 22 | 0.81% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.81% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.78% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 28 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


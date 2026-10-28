# JVM hotspots (JFR ExecutionSample)

Samples: 15021

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.80% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.51% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 12 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.28% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 18 | 1.08% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 19 | 1.05% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 1.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.93% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 23 | 0.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.69% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.60% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.58% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.57% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 32 | 0.54% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 33 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.53% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 35 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 41 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 44 | 0.38% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 47 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 48 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.31% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 52 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 53 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 54 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 55 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.26% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 212` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 59 | 0.26% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 60 | 0.26% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 4.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 4.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 7 | 2.85% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 8 | 2.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 2.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.78% | `java.util.ArrayList.toArray() line: 370` |
| 11 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 15 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.08% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.06% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 1.00% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.94% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 24 | 0.83% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 25 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.80% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 27 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.79% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 29 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


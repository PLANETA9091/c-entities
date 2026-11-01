# JVM hotspots (JFR ExecutionSample)

Samples: 15205

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 8 | 1.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 9 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.27% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 15 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 17 | 1.08% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 20 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 21 | 0.92% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 26 | 0.68% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 27 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 28 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 33 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 34 | 0.58% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 39 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 42 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.37% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 47 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 48 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 49 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.34% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.32% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 55 | 0.32% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 56 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 59 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.37% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 6.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 3 | 5.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 4.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 4.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 3.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.87% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 8 | 2.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 9 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 12 | 1.39% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 14 | 1.27% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 15 | 1.18% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.16% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 17 | 1.10% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 18 | 1.09% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 19 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 20 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.92% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 24 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.75% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 27 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.68% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |

GC pauses: 0, total 0 ms


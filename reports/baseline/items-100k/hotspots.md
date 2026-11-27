# JVM hotspots (JFR ExecutionSample)

Samples: 17501

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.91% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 1.19% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 18 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.98% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.75% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 25 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 27 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 29 | 0.59% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 30 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 32 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 36 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 39 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 43 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 44 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.35% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 47 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 50 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 51 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 56 | 0.25% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 495` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 58 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 59 | 0.22% | `java.lang.Double.isFinite(double) line: 1023` |
| 60 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 2 | 4.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 3 | 4.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 3.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 8 | 1.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 9 | 1.71% | `java.util.ArrayList.toArray() line: 370` |
| 10 | 1.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 11 | 1.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 14 | 1.23% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.21% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 16 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 17 | 1.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.08% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 19 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.86% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |
| 25 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 26 | 0.82% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 27 | 0.81% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.71% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1691` |

GC pauses: 0, total 0 ms


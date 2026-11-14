# JVM hotspots (JFR ExecutionSample)

Samples: 12950

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 2.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 6 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.94% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 8 | 1.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.12% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 17 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 19 | 1.07% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 20 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 21 | 0.90% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.81% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 27 | 0.66% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1845` |
| 29 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 32 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 36 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.getXRot() line: 5297` |
| 41 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 43 | 0.39% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 46 | 0.38% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 24` |
| 47 | 0.37% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 50 | 0.34% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 52 | 0.33% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 28` |
| 53 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1296` |
| 55 | 0.30% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 27` |
| 56 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 58 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 60 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 5.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.88% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 5 | 2.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.02% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 8 | 1.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 11 | 1.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 178` |
| 14 | 1.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 16 | 1.22% | `java.util.ArrayList.toArray() line: 370` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1563` |
| 18 | 1.12% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.97% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |
| 22 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.83% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.77% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 29 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.73% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1542` |

GC pauses: 0, total 0 ms


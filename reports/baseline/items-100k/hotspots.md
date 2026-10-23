# JVM hotspots (JFR ExecutionSample)

Samples: 17394

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 4.06% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 2.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 11 | 1.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 13 | 1.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 16 | 1.18% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 17 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 18 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 19 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 22 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 23 | 0.95% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 24 | 0.85% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 26 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 28 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.62% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 31 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 32 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 33 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 34 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 36 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 38 | 0.53% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 39 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 40 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 41 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 42 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 43 | 0.41% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 44 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 45 | 0.39% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 46 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 50 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 51 | 0.33% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 52 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 53 | 0.30% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 54 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |
| 55 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 59 | 0.27% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 60 | 0.27% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 6.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 5.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 5.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.52% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 2.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 12 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 16 | 1.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 1.22% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 18 | 1.17% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 1.10% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 20 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 21 | 1.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 24 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.70% | `java.util.ArrayList.toArray() line: 369` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |

GC pauses: 0, total 0 ms


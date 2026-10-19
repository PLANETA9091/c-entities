# JVM hotspots (JFR ExecutionSample)

Samples: 13379

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.87% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 4.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.97% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 2.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 2.58% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 13 | 1.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 15 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 16 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 17 | 1.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 1.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 19 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 1.04% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 21 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 22 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 23 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 24 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 25 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 26 | 0.78% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 27 | 0.76% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 29 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 31 | 0.59% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 33 | 0.58% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 34 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 35 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 36 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 37 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 38 | 0.49% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 39 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 40 | 0.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 41 | 0.40% | `net.minecraft.server.MinecraftServer.moonrise$executeMidTickTasks() line: 390` |
| 42 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 43 | 0.38% | `java.util.ArrayDeque.sub(int, int, int) line: 248` |
| 44 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 46 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 47 | 0.31% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 48 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 49 | 0.30% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 50 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 52 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 53 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 54 | 0.28% | `java.util.ArrayList$Itr.next() line: 1057` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 56 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.objects.AbstractObjectList.ensureIndex(int) line: 54` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 60 | 0.24% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 7.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 3 | 6.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 5.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 5.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.97% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 2.90% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.57% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 10 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 2.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.85% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 1.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 17 | 1.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 1.29% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 1.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 1.04% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 23 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 28 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 187` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |

GC pauses: 0, total 0 ms


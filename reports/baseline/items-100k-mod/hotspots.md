# JVM hotspots (JFR ExecutionSample)

Samples: 14775

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 4.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 5 | 3.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 7 | 2.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 2.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 2.00% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.94% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.15% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 16 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 19 | 0.87% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 20 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.85% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.70% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 25 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 26 | 0.62% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 27 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 28 | 0.61% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 31 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 32 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 33 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 34 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 36 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.44% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 38 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 40 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 41 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 430` |
| 42 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 44 | 0.35% | `java.util.ArrayList$Itr.next() line: 1057` |
| 45 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 46 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 47 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 48 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1008` |
| 50 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 51 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 52 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 53 | 0.30% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 194` |
| 55 | 0.28% | `java.lang.ThreadLocal.get() line: 172` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 59 | 0.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 60 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 333` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 4.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 3.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.96% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 8 | 2.34% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 9 | 2.31% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 10 | 2.29% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 2.26% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 2.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 2.00% | `java.util.HashMap.containsKey(Object) line: 602` |
| 14 | 1.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 15 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.18% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 17 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 18 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.96% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 21 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 23 | 0.89% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 188` |
| 24 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 26 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 27 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.79% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 30 | 0.75% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |

GC pauses: 0, total 0 ms


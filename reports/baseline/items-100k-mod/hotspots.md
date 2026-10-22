# JVM hotspots (JFR ExecutionSample)

Samples: 12086

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.44% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 5.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 4.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 10 | 1.64% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 15 | 1.31% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 16 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 18 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 19 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.87% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 21 | 0.86% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.82% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.76% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 24 | 0.68% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 27 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.61% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.60% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 30 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 32 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 33 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 34 | 0.57% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.56% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 36 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 38 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 40 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 42 | 0.45% | `java.util.EnumMap.get(Object) line: 247` |
| 43 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 44 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 45 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 46 | 0.36% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |
| 47 | 0.34% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 48 | 0.33% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 49 | 0.33% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 50 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 51 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 351` |
| 52 | 0.31% | `java.util.ArrayDeque.sub(int, int, int) line: 248` |
| 53 | 0.31% | `java.lang.ThreadLocal.get() line: 172` |
| 54 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 55 | 0.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 56 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 58 | 0.29% | `java.util.ArrayList$Itr.next() line: 1057` |
| 59 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasOnlyAir() line: 173` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 6.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 5.29% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.42% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 2.10% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 2.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.98% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.92% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.64% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 17 | 1.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 1.06% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 22 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.88% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 24 | 0.86% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 25 | 0.81% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.65% | `java.util.ArrayList.toArray() line: 369` |
| 28 | 0.65% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 225` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 17645

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 4.80% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 4.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.02% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 9 | 1.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 11 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 13 | 1.38% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 16 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 17 | 1.04% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 19 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.96% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 21 | 0.90% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 22 | 0.89% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.86% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 24 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 27 | 0.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 30 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 31 | 0.65% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 32 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 33 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 35 | 0.56% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 36 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 37 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 40 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 41 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 45 | 0.34% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 46 | 0.33% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 47 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 48 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 50 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 52 | 0.31% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 54 | 0.29% | `java.lang.ThreadLocal.get() line: 172` |
| 55 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 56 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 57 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 58 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 59 | 0.26% | `java.util.ArrayList$Itr.next() line: 1057` |
| 60 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 4.19% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.95% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 2.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 2.02% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 11 | 1.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 13 | 1.64% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 14 | 1.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.09% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 17 | 1.07% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 19 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 21 | 0.94% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 22 | 0.93% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 23 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 25 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 28 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.72% | `java.util.ArrayList.toArray() line: 369` |

GC pauses: 0, total 0 ms


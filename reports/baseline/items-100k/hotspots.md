# JVM hotspots (JFR ExecutionSample)

Samples: 18814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.96% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 4.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 10 | 1.57% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.28% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 15 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.16% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 18 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 19 | 1.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 1.01% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 21 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 22 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 23 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 24 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 25 | 0.72% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.71% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 27 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 28 | 0.65% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 31 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 32 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 33 | 0.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 35 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 38 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 39 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.45% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 42 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 43 | 0.44% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 44 | 0.43% | `java.util.ArrayList$Itr.next() line: 1057` |
| 45 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 46 | 0.40% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 48 | 0.37% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.getTicksFrozen() line: 3632` |
| 50 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 51 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 52 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 53 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 54 | 0.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 56 | 0.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 58 | 0.28% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 59 | 0.28% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 7.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 3 | 7.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 5.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 3.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.80% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 9 | 1.83% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.58% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.57% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.28% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 15 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 19 | 1.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 21 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.92% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 25 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.82% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |

GC pauses: 0, total 0 ms


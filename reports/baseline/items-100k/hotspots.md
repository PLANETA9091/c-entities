# JVM hotspots (JFR ExecutionSample)

Samples: 15571

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.83% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 2.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 2.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 2.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 2.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.67% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 1.56% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 12 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 13 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 15 | 1.28% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 17 | 1.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 18 | 1.23% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 19 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 20 | 1.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 21 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 1.10% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 24 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 25 | 0.88% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.76% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 28 | 0.75% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 29 | 0.75% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 30 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 31 | 0.67% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 32 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 34 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 35 | 0.61% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.61% | `java.util.HashMap.getNode(Object) line: 585` |
| 37 | 0.57% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 38 | 0.55% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 39 | 0.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 40 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 41 | 0.51% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 42 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 43 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 44 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 45 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 46 | 0.44% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 48 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 50 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 52 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 54 | 0.40% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 55 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 56 | 0.39% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 25` |
| 57 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 58 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 59 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 4.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 3 | 4.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 3.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 3.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 10 | 1.86% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 11 | 1.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 13 | 1.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 14 | 1.43% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 1.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 1.31% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 17 | 1.28% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 18 | 1.28% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 19 | 1.21% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 1.20% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 21 | 1.10% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 22 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 23 | 1.09% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 24 | 1.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 27 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.86% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 29 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.80% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |

GC pauses: 0, total 0 ms


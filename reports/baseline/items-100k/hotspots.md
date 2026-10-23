# JVM hotspots (JFR ExecutionSample)

Samples: 13123

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 4.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 4.01% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 3.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 2.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.49% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 2.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 16 | 1.47% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 17 | 1.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 18 | 1.31% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 19 | 1.15% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 20 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 21 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 22 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 24 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 27 | 0.71% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 28 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 29 | 0.60% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 30 | 0.60% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 31 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 32 | 0.59% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 33 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 35 | 0.54% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 36 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 37 | 0.50% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 38 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 39 | 0.47% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 40 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 41 | 0.46% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.40% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 45 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 46 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 48 | 0.35% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 49 | 0.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.31% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 51 | 0.30% | `java.lang.ThreadLocal.get() line: 172` |
| 52 | 0.30% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 54 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 56 | 0.28% | `it.unimi.dsi.fastutil.objects.AbstractObjectList.ensureIndex(int) line: 54` |
| 57 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 58 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 60 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 6.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 5.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 4.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 3.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 3.05% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.49% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 10 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 2.43% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 2.10% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.55% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 16 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.36% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 18 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 20 | 1.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 1.15% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 22 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 23 | 1.04% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 28 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 187` |
| 29 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.63% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 167` |

GC pauses: 0, total 0 ms


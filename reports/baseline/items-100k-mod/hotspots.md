# JVM hotspots (JFR ExecutionSample)

Samples: 8641

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 8.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 8.74% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 7.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.23% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 7 | 3.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 8 | 1.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 10 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.00% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 15 | 0.98% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 219` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 175` |
| 17 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 18 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 19 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 20 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.74% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 22 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 25 | 0.61% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 26 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 27 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 28 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.56% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 30 | 0.51% | `java.util.ArrayList$Itr.next() line: 1057` |
| 31 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 34 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 36 | 0.39% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 37 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 40 | 0.30% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 41 | 0.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 42 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 43 | 0.28% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 44 | 0.28% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 45 | 0.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.25% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 47 | 0.25% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 48 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 49 | 0.24% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 50 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 444` |
| 51 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 52 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 53 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 4982` |
| 54 | 0.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 56 | 0.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 57 | 0.19% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 58 | 0.19% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 59 | 0.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 60 | 0.19% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 10.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 10.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 7.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 6.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.23% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 7 | 2.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 2.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 9 | 1.75% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 10 | 1.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 1.31% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 1.22% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 14 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 15 | 1.12% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 107` |
| 16 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 17 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 21 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 24 | 0.57% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 167` |
| 25 | 0.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 26 | 0.56% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 27 | 0.50% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 162` |
| 28 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.39% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 30 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |

GC pauses: 0, total 0 ms


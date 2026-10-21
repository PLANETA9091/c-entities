# JVM hotspots (JFR ExecutionSample)

Samples: 9097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 7.39% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 6.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 5.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.90% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 3.66% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 8 | 3.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 10 | 2.56% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 663` |
| 11 | 1.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 12 | 1.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 14 | 1.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 15 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 16 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 17 | 1.19% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 1.08% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 20 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1126` |
| 23 | 0.67% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 664` |
| 24 | 0.59% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 26 | 0.53% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 27 | 0.52% | `java.util.ArrayList$Itr.next() line: 1057` |
| 28 | 0.45% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 131` |
| 29 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 219` |
| 30 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 31 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.38% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 33 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 34 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 35 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 36 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |
| 37 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 38 | 0.32% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 39 | 0.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 40 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 41 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 42 | 0.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2183` |
| 43 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 44 | 0.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 46 | 0.26% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 47 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 135` |
| 49 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 50 | 0.24% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 51 | 0.22% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 52 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 183` |
| 54 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 56 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 58 | 0.20% | `net.minecraft.world.entity.Entity.getDeltaMovement() line: 4877` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1123` |
| 60 | 0.19% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 9.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 8.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 7.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 5.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 4.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 3.90% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 3.65% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 9 | 3.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1135` |
| 10 | 1.93% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 107` |
| 12 | 1.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 13 | 1.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 931` |
| 16 | 1.19% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 17 | 1.18% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 18 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 0.75% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 167` |
| 23 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 24 | 0.57% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1275` |
| 25 | 0.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 27 | 0.51% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 162` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 29 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 30 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms


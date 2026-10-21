# JVM hotspots (JFR ExecutionSample)

Samples: 9146

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 6.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 5.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 5.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 5.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 7 | 3.05% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.84% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 9 | 2.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 2.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 11 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 12 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 219` |
| 14 | 0.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 16 | 0.93% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 0.89% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 175` |
| 19 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 20 | 0.74% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 21 | 0.70% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 22 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 23 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 26 | 0.57% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 27 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1008` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 31 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 32 | 0.47% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 33 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 34 | 0.39% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 35 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.32% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 38 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 159` |
| 40 | 0.30% | `java.util.ArrayList$Itr.next() line: 1057` |
| 41 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 42 | 0.28% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 43 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 44 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.25% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 46 | 0.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 47 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.23% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 49 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 51 | 0.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2183` |
| 52 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 53 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 54 | 0.21% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 56 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 59 | 0.20% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 179` |
| 60 | 0.20% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 158` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 2 | 7.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 6.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 6.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 6.31% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 6 | 6.27% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 7 | 3.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 3.03% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 2.82% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 10 | 2.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 1.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.82% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 16 | 1.15% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 17 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 0.93% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 107` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 20 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1158` |
| 24 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 25 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 26 | 0.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 27 | 0.47% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |
| 29 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms


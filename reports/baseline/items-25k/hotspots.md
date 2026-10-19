# JVM hotspots (JFR ExecutionSample)

Samples: 3229

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 2 | 4.34% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 4.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 9 | 1.95% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 14 | 1.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 16 | 1.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 17 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 18 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 19 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 20 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.81% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.81% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 25 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 26 | 0.68% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 28 | 0.65% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 30 | 0.65% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 31 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 32 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 34 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 36 | 0.53% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 285` |
| 37 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 40 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 41 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 42 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 43 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.40% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 45 | 0.40% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 46 | 0.40% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 286` |
| 47 | 0.37% | `java.util.ArrayDeque.size() line: 648` |
| 48 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 49 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 297` |
| 50 | 0.34% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 51 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 53 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 55 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 57 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 58 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 6.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 4.46% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 3.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.73% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 2.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 10 | 2.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.95% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.64% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 16 | 1.27% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 17 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 19 | 1.08% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 20 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 21 | 0.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.81% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 27 | 0.68% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 28 | 0.68% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |

GC pauses: 0, total 0 ms


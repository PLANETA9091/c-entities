# JVM hotspots (JFR ExecutionSample)

Samples: 3762

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 2 | 3.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 3 | 2.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 5 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.65% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 9 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 10 | 1.52% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.06% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 15 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 17 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.98% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 21 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.85% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.82% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.82% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 26 | 0.80% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 27 | 0.66% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 28 | 0.56% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 32 | 0.51% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 33 | 0.51% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 34 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 37 | 0.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 39 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 41 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 44 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 45 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 46 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 47 | 0.35% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 48 | 0.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 49 | 0.32% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 50 | 0.32% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 52 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 53 | 0.29% | `java.lang.String.hashCode() line: 2358` |
| 54 | 0.29% | `net.minecraft.world.level.levelgen.SurfaceRules$SequenceRule.tryApply(int, int, int) line: 585` |
| 55 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 56 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 57 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 58 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.29% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 3.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 2.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 5 | 2.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.31% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.21% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 2.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 10 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 12 | 1.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.52% | `java.util.HashMap.containsKey(Object) line: 602` |
| 14 | 1.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 18 | 1.06% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 20 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 0.98% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 0.90% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 23 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 24 | 0.85% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 25 | 0.82% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.80% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 27 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.69% | `java.util.ArrayList.toArray() line: 369` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |

GC pauses: 0, total 0 ms


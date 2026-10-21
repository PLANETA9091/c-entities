# JVM hotspots (JFR ExecutionSample)

Samples: 4538

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.79% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 3.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 1.85% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.26% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.12% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 1.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.84% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 18 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 20 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.79% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 22 | 0.77% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 23 | 0.75% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 24 | 0.75% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 27 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 28 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 29 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 30 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 31 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 32 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 34 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 37 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 38 | 0.40% | `java.util.Collections.unmodifiableCollection(Collection) line: 1047` |
| 39 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 41 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 42 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 43 | 0.37% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 44 | 0.35% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 45 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 46 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 47 | 0.35% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 48 | 0.35% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 49 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 52 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 53 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 54 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 55 | 0.31% | `org.objectweb.asm.SymbolTable.addConstantUtf8(String) line: 798` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 57 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 58 | 0.29% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 59 | 0.29% | `java.lang.ThreadLocal.get() line: 172` |
| 60 | 0.26% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 4.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.39% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 3.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 8 | 1.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.85% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 1.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 12 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 14 | 1.34% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 19 | 0.88% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 20 | 0.88% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 23 | 0.79% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 26 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 27 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 29 | 0.57% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 543` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |

GC pauses: 0, total 0 ms


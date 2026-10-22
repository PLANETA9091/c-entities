# JVM hotspots (JFR ExecutionSample)

Samples: 2993

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 2 | 3.61% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 3 | 3.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 3.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 2.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 8 | 2.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.50% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.44% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 13 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 16 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 17 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 21 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 22 | 0.94% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 23 | 0.90% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 24 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 26 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 27 | 0.77% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 28 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 29 | 0.70% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 30 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.63% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 32 | 0.63% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 33 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 34 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 35 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 36 | 0.53% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 37 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.53% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.50% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 40 | 0.47% | `java.util.EnumMap.get(Object) line: 247` |
| 41 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.43% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 45 | 0.40% | `java.util.ArrayDeque.size() line: 648` |
| 46 | 0.37% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.37% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 49 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 50 | 0.33% | `net.minecraft.world.level.levelgen.synth.ImprovedNoise.gradDot(int, double, double, double) line: 78` |
| 51 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 52 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 54 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 55 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3541` |
| 56 | 0.30% | `java.lang.String.hashCode() line: 2358` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 414` |
| 59 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 285` |
| 60 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 2 | 4.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 4.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 3.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.41% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 2.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 2.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 11 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 14 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 16 | 1.44% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 17 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.30% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 19 | 1.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 21 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 22 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 23 | 0.97% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 24 | 0.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1483` |
| 25 | 0.94% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 26 | 0.90% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 27 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 30 | 0.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |

GC pauses: 0, total 0 ms


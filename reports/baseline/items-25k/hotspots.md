# JVM hotspots (JFR ExecutionSample)

Samples: 2559

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 5.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 6 | 2.19% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 7 | 1.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 10 | 1.64% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.60% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 12 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 1.17% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 16 | 1.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 18 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 1.02% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 20 | 1.02% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 21 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 24 | 0.78% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 25 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |
| 26 | 0.74% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 28 | 0.66% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.59% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 31 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 32 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 33 | 0.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 34 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.47% | `java.util.ArrayDeque.sub(int, int, int) line: 248` |
| 36 | 0.43% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 37 | 0.43% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 38 | 0.43% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 40 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 41 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 45 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 46 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 47 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 49 | 0.35% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 50 | 0.35% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.35% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 52 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 53 | 0.31% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 194` |
| 54 | 0.31% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 55 | 0.31% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 56 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 57 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer$Data.moonrise$getPalette() line: 458` |
| 58 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.swapPalette(int[], IntUnaryOperator) line: 376` |
| 59 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer$Data.copyFrom(Palette, BitStorage) line: 468` |
| 60 | 0.27% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 6.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 5.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 2.23% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 8 | 1.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 10 | 1.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 1.76% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 12 | 1.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.17% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 16 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 18 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 19 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 20 | 1.02% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 23 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.66% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 30 | 0.55% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1725` |

GC pauses: 0, total 0 ms


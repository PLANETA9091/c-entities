# JVM hotspots (JFR ExecutionSample)

Samples: 7663

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 2.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 7 | 2.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 8 | 2.22% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 9 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 11 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.68% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 13 | 1.51% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 14 | 1.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 16 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 19 | 1.19% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 20 | 1.17% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 21 | 1.04% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 22 | 1.03% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 23 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 24 | 0.84% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2188` |
| 26 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.78% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 28 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.65% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 31 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 32 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 33 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 35 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 36 | 0.54% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 37 | 0.51% | `java.util.EnumMap.get(Object) line: 247` |
| 38 | 0.51% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1271` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 40 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 41 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 43 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 797` |
| 44 | 0.40% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 45 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 46 | 0.40% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 26` |
| 47 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 396` |
| 50 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 51 | 0.34% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 25` |
| 52 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 53 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 54 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 55 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 56 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 227` |
| 57 | 0.29% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 58 | 0.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2184` |
| 59 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 60 | 0.27% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 4.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 4 | 3.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 6 | 2.84% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 2.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 2.51% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 9 | 2.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 10 | 2.24% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 11 | 1.93% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 12 | 1.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 13 | 1.77% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 14 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.41% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 16 | 1.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 17 | 1.33% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 18 | 1.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 20 | 1.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 21 | 1.11% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 1.08% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 23 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.97% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.94% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.90% | `net.minecraft.world.entity.Entity.setOldRot() line: 2178` |
| 27 | 0.81% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 187` |
| 28 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 30 | 0.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |

GC pauses: 0, total 0 ms


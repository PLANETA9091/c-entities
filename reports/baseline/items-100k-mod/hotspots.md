# JVM hotspots (JFR ExecutionSample)

Samples: 8902

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 4.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 2.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.65% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 6 | 2.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 7 | 2.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 9 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 14 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.25% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 16 | 1.10% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 17 | 0.95% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 18 | 0.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.92% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 20 | 0.91% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 21 | 0.89% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 23 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 25 | 0.82% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 26 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 27 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.70% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 29 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.62% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.54% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 37 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 38 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.48% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 42 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 43 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 44 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 47 | 0.38% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 48 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 49 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 50 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 51 | 0.35% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 53 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 54 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 55 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 56 | 0.27% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 57 | 0.26% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 58 | 0.26% | `net.minecraft.util.Mth.lerp3(double, double, double, double, double, double, double, double, double, double, double) line: 561` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 60 | 0.26% | `java.util.Objects.equals(Object, Object) line: 64` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 5.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 3.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 2.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.65% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 7 | 2.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.31% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 10 | 1.64% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 11 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.51% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 13 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 16 | 1.28% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 17 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 19 | 1.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 21 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 22 | 0.89% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |
| 25 | 0.86% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 26 | 0.85% | `java.util.ArrayList.toArray() line: 369` |
| 27 | 0.85% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1725` |
| 28 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 29 | 0.81% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 30 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


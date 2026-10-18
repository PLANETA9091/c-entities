# JVM hotspots (JFR ExecutionSample)

Samples: 7891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 5.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 3.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.46% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.79% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.76% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 12 | 1.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 16 | 1.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 17 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 18 | 1.20% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 1.08% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 21 | 0.90% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 22 | 0.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 23 | 0.75% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 24 | 0.70% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 26 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 27 | 0.67% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 28 | 0.66% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 30 | 0.65% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 32 | 0.58% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 33 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 34 | 0.53% | `java.util.EnumMap.get(Object) line: 247` |
| 35 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 36 | 0.48% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 37 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 39 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 42 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 43 | 0.44% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 46 | 0.41% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 48 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 50 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 51 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 53 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 320` |
| 54 | 0.30% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 55 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 56 | 0.29% | `net.minecraft.server.level.ServerLevel.getChunkSource() line: 1823` |
| 57 | 0.29% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 58 | 0.28% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 59 | 0.28% | `net.minecraft.world.level.material.FluidState.getType() line: 58` |
| 60 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 6.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 6.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.52% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 2.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 2.46% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 9 | 2.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.89% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 12 | 1.81% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 13 | 1.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 1.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 16 | 1.44% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 17 | 1.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 18 | 1.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 1.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 1.20% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |
| 21 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 22 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 23 | 0.98% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 24 | 0.96% | `java.util.ArrayList.toArray() line: 369` |
| 25 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.90% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 27 | 0.82% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4790` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 10977

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 3 | 5.86% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 3.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.91% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 2.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.89% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 12 | 1.49% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 13 | 1.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 14 | 1.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 16 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 17 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 20 | 0.79% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 160` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.70% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 25 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 355` |
| 27 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 29 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 30 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 31 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 71` |
| 32 | 0.53% | `java.util.EnumMap.get(Object) line: 247` |
| 33 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 35 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 36 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 37 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 38 | 0.46% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 39 | 0.46% | `net.minecraft.world.entity.EntityType.getCategory() line: 1335` |
| 40 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 797` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.clear() line: 493` |
| 42 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 44 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 45 | 0.40% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 47 | 0.39% | `it.unimi.dsi.fastutil.objects.AbstractObjectList.ensureIndex(int) line: 54` |
| 48 | 0.38% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 49 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2183` |
| 51 | 0.36% | `java.util.ArrayDeque.sub(int, int, int) line: 248` |
| 52 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 120` |
| 53 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 54 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 292` |
| 55 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 57 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 475` |
| 59 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 8.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 3 | 6.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 4 | 4.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 3.52% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 6 | 2.90% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 7 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.08% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 10 | 1.89% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 11 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.79% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 1.49% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 16 | 1.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 17 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 20 | 1.17% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 21 | 1.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 24 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1062` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.add(Object) line: 374` |
| 30 | 0.62% | `net.minecraft.world.level.Level.noCollision(Entity, AABB) line: 1502` |

GC pauses: 0, total 0 ms


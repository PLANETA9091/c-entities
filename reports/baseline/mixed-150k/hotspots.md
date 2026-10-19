# JVM hotspots (JFR ExecutionSample)

Samples: 103439

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 2.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 16 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.61% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.55% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.51% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 24 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 26 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.45% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 30 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 31 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 32 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 34 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.34% | `java.lang.ThreadLocal.get() line: 172` |
| 36 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 37 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 41 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 42 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 44 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 46 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 47 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 48 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 49 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 50 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 52 | 0.22% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 53 | 0.21% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 54 | 0.21% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 55 | 0.21% | `java.util.EnumMap.get(Object) line: 247` |
| 56 | 0.20% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 58 | 0.20% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 59 | 0.19% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 60 | 0.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.20% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.07% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.83% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 14 | 0.65% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 16 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 17 | 0.55% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 19 | 0.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.52% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 21 | 0.51% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 22 | 0.48% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 24 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 26 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 27 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |

GC pauses: 0, total 0 ms


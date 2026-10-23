# JVM hotspots (JFR ExecutionSample)

Samples: 69518

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.15% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 11 | 1.14% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 15 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 19 | 0.91% | `java.lang.Enum.ordinal() line: 128` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 21 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 24 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.79% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 27 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.62% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 32 | 0.61% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 33 | 0.55% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.51% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 37 | 0.50% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 38 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 39 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 41 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 42 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 47 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 50 | 0.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 52 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 55 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 58 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 59 | 0.29% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 60 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 7 | 1.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 8 | 1.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 14 | 1.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.15% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 19 | 0.91% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.81% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 23 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 29 | 0.60% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 30 | 0.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |

GC pauses: 0, total 0 ms


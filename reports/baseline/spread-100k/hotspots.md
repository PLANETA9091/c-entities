# JVM hotspots (JFR ExecutionSample)

Samples: 79545

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 9 | 1.08% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 11 | 1.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.97% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 13 | 0.95% | `java.lang.Enum.ordinal() line: 128` |
| 14 | 0.91% | `java.lang.ThreadLocal.get() line: 172` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 17 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 18 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 19 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 21 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 27 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 29 | 0.73% | `java.util.HashMap.getNode(Object) line: 585` |
| 30 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 31 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 32 | 0.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 33 | 0.56% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 34 | 0.53% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 35 | 0.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 40 | 0.45% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 42 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 43 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 46 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 47 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 48 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 50 | 0.33% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 51 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 53 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 54 | 0.32% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 57 | 0.30% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 58 | 0.30% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 59 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 60 | 0.29% | `java.util.EnumMap.get(Object) line: 247` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.47% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.97% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 6 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 7 | 1.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 8 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.38% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 14 | 1.09% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.97% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 18 | 0.90% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 19 | 0.77% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 21 | 0.75% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 22 | 0.74% | `java.util.HashMap.containsKey(Object) line: 602` |
| 23 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.71% | `java.util.EnumMap.get(Object) line: 247` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 27 | 0.68% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 29 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 64367

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 7 | 1.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 8 | 1.38% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 16 | 1.08% | `java.lang.Enum.ordinal() line: 128` |
| 17 | 1.05% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 19 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 21 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 23 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 26 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 29 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 31 | 0.60% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 33 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 34 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.51% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 38 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 39 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 40 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 41 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 42 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 43 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 45 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 46 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 47 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 48 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 50 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 54 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 57 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 58 | 0.30% | `net.minecraft.world.entity.Mob.checkDespawn() line: 715` |
| 59 | 0.30% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 60 | 0.29% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.28% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 5 | 2.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.77% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 10 | 1.38% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 12 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 14 | 1.28% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 16 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 17 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.05% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 19 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 21 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.66% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 24 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 26 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 28 | 0.62% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 30 | 0.61% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


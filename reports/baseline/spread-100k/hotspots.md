# JVM hotspots (JFR ExecutionSample)

Samples: 69568

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.replaceFrom(AttributeInstance) line: 181` |
| 11 | 1.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 1.07% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.86% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 23 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 24 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 26 | 0.75% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 28 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.66% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.62% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 33 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 34 | 0.61% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 35 | 0.61% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 38 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 40 | 0.47% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 41 | 0.46% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 43 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 45 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 46 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 48 | 0.40% | `java.util.EnumMap.get(Object) line: 247` |
| 49 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 50 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 52 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 54 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 55 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 56 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 57 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 60 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.93% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 7 | 1.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.39% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeSupplier.createInstance(Consumer, Holder) line: 50` |
| 17 | 1.08% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 1.02% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 22 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.76% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 29 | 0.61% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 30 | 0.55% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |

GC pauses: 0, total 0 ms


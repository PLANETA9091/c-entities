# JVM hotspots (JFR ExecutionSample)

Samples: 93443

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.39% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 12 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.08% | `java.lang.Enum.ordinal() line: 128` |
| 16 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 18 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 19 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 21 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 23 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 26 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 27 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 28 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 31 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.65% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 34 | 0.65% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 35 | 0.58% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 36 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 37 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 39 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 40 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 43 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 45 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 46 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 47 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 48 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 49 | 0.41% | `java.lang.ThreadLocal.get() line: 172` |
| 50 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 53 | 0.36% | `java.util.EnumMap.get(Object) line: 247` |
| 54 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 56 | 0.36% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 58 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 59 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 60 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.02% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 9 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 1.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 11 | 1.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.52% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 14 | 1.46% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.40% | `java.util.HashMap.containsKey(Object) line: 602` |
| 16 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 18 | 1.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 20 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.88% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.87% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 25 | 0.69% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 28 | 0.65% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |

GC pauses: 0, total 0 ms


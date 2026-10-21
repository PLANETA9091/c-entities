# JVM hotspots (JFR ExecutionSample)

Samples: 51902

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 2.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 12 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 14 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 1.26% | `java.util.HashMap.getNode(Object) line: 585` |
| 17 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.20% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 19 | 1.09% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 21 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 23 | 0.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 26 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 27 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 29 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 31 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 32 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 33 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 36 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 37 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 38 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 39 | 0.55% | `java.lang.Enum.ordinal() line: 128` |
| 40 | 0.55% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 41 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 42 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 43 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 45 | 0.46% | `java.util.ArrayDeque.size() line: 648` |
| 46 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 48 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 49 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 50 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 51 | 0.41% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 52 | 0.40% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 54 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 55 | 0.40% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 57 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 58 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 59 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 60 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 2.34% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 2.28% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.59% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 12 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 15 | 1.27% | `java.util.HashMap.containsKey(Object) line: 602` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 18 | 1.19% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 19 | 1.09% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 20 | 1.07% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 21 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 26 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 29 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 30 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |

GC pauses: 0, total 0 ms


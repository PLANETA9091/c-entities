# JVM hotspots (JFR ExecutionSample)

Samples: 53667

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 2.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.30% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 16 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 22 | 0.83% | `java.util.HashMap.getNode(Object) line: 585` |
| 23 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 29 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 30 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 31 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 32 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 35 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 36 | 0.61% | `java.lang.Enum.ordinal() line: 128` |
| 37 | 0.58% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 38 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 39 | 0.56% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 40 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 41 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 42 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 44 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 45 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 46 | 0.49% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 47 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 48 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 49 | 0.49% | `java.util.ArrayDeque.size() line: 648` |
| 50 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 51 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 52 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 53 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 54 | 0.42% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 55 | 0.41% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 56 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 57 | 0.40% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 58 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 59 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 60 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.21% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 2.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 9 | 1.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.70% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 13 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 15 | 1.30% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 17 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 18 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.97% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.96% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 25 | 0.83% | `java.util.HashMap.containsKey(Object) line: 602` |
| 26 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 29 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 30 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 51226

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 2.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.65% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 15 | 1.13% | `java.util.HashMap.getNode(Object) line: 585` |
| 16 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 21 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 23 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 26 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 29 | 0.69% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 30 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 31 | 0.66% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 32 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 34 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 35 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 37 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 38 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 39 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.54% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.53% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 42 | 0.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 43 | 0.49% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 44 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 45 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 47 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 48 | 0.46% | `java.util.ArrayDeque.size() line: 648` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 50 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 51 | 0.45% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 52 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 53 | 0.44% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 54 | 0.44% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 55 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 56 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 57 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 58 | 0.42% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 59 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 60 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 4 | 3.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 2.29% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 2.19% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 9 | 1.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.66% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 12 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 13 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.25% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 1.14% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 18 | 1.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.99% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 0.97% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 25 | 0.89% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 26 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 28 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.75% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


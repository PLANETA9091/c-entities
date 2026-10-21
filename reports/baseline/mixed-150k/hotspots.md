# JVM hotspots (JFR ExecutionSample)

Samples: 101360

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.20% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.16% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.91% | `java.lang.ThreadLocal.get() line: 172` |
| 15 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 0.69% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 25 | 0.68% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 26 | 0.66% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 27 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 28 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 29 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 31 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 32 | 0.51% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 36 | 0.39% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 37 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 39 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 40 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 41 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 42 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 45 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 47 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 86` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 51 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 54 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 55 | 0.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 56 | 0.23% | `java.util.EnumMap.get(Object) line: 247` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 58 | 0.22% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 59 | 0.22% | `java.util.ArrayDeque.size() line: 648` |
| 60 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 1.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.26% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.17% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 13 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 16 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.78% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 18 | 0.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.69% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 22 | 0.66% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.57% | `java.util.EnumMap.get(Object) line: 247` |
| 26 | 0.54% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 27 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 774` |
| 28 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.40% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms


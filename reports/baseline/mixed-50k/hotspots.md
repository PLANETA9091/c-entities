# JVM hotspots (JFR ExecutionSample)

Samples: 13534

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 3.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.28% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 2.11% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 11 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.16% | `java.lang.ThreadLocal.get() line: 172` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.00% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 19 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 0.90% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 21 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 24 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 27 | 0.71% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 28 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 30 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 33 | 0.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 37 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 38 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.47% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 41 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 45 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 47 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 48 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 49 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 51 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 53 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 54 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 56 | 0.33% | `java.util.ArrayDeque.size() line: 648` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 58 | 0.33% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 59 | 0.33% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 60 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 3.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 2.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 7 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.10% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 13 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 16 | 1.14% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 1.07% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 1.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 1.01% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.92% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 23 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 26 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 28 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |

GC pauses: 0, total 0 ms


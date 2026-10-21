# JVM hotspots (JFR ExecutionSample)

Samples: 77177

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.07% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.25% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 0.98% | `java.lang.ThreadLocal.get() line: 172` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 17 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 19 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 21 | 0.83% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.81% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 23 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.81% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 25 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 27 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 30 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 31 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 32 | 0.49% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 33 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 34 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 36 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 37 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.38% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 41 | 0.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 42 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 45 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 47 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 48 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 49 | 0.33% | `java.util.ArrayDeque.size() line: 648` |
| 50 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 51 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 52 | 0.32% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 53 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 54 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 57 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 58 | 0.27% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 59 | 0.27% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 60 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.08% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 9 | 1.76% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 11 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.04% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 13 | 0.97% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 14 | 0.95% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 15 | 0.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 18 | 0.89% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.81% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.81% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 24 | 0.76% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 26 | 0.63% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 29 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 30 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


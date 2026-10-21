# JVM hotspots (JFR ExecutionSample)

Samples: 87811

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.85% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 1.09% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 16 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |
| 20 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 22 | 0.81% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 27 | 0.56% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 28 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 33 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.41% | `java.lang.Enum.ordinal() line: 128` |
| 37 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 38 | 0.40% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 39 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |
| 41 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 42 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.33% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 44 | 0.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 45 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 46 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 48 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 49 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 50 | 0.28% | `java.util.ArrayDeque.size() line: 648` |
| 51 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 52 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 53 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 55 | 0.25% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 56 | 0.24% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 57 | 0.24% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 58 | 0.24% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 60 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 3.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.88% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.27% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 12 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 13 | 1.10% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 14 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 15 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.83% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.82% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.79% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 25 | 0.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |

GC pauses: 0, total 0 ms


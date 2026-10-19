# JVM hotspots (JFR ExecutionSample)

Samples: 44927

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.22% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.20% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.13% | `java.lang.ThreadLocal.get() line: 172` |
| 16 | 1.07% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 19 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 20 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 22 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 25 | 0.70% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 26 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 27 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 35 | 0.46% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 36 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 37 | 0.41% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 38 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 39 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 41 | 0.39% | `java.util.ArrayDeque.size() line: 648` |
| 42 | 0.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 44 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 45 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 46 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 48 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 49 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 50 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 53 | 0.32% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 54 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 55 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 57 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 60 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 1.84% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.22% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.21% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 14 | 1.03% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.82% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 20 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 21 | 0.71% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 22 | 0.69% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.58% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 27 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.55% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 30 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |

GC pauses: 0, total 0 ms


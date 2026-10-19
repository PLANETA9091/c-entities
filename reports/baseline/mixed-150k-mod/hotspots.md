# JVM hotspots (JFR ExecutionSample)

Samples: 66339

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.21% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.88% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.29% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 19 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 23 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 26 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 27 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 31 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 34 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 35 | 0.49% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 36 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 37 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 38 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 39 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 41 | 0.40% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 42 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 46 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 47 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 48 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 50 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 51 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 53 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 54 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 55 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 56 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 59 | 0.26% | `java.lang.ThreadLocal.get() line: 172` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.AbstractObjectList.ensureIndex(int) line: 54` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.21% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 2.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.87% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.29% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.93% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 17 | 0.87% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 19 | 0.82% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 22 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 24 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |

GC pauses: 0, total 0 ms


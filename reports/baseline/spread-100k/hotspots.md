# JVM hotspots (JFR ExecutionSample)

Samples: 70801

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.73% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.26% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 1.21% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 14 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.06% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 17 | 1.06% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 1.05% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 28 | 0.73% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 31 | 0.65% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.64% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 33 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 34 | 0.57% | `java.util.EnumMap.get(Object) line: 247` |
| 35 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 37 | 0.51% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 38 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 39 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 41 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 42 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 46 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 47 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 49 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 50 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 53 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 55 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 56 | 0.36% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.35% | `java.lang.ThreadLocal.get() line: 172` |
| 59 | 0.34% | `java.util.ArrayDeque.size() line: 648` |
| 60 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 2.05% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 8 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.73% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.45% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 1.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 16 | 1.23% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 18 | 1.06% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 19 | 1.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.69% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 30 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


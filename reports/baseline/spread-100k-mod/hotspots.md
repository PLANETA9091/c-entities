# JVM hotspots (JFR ExecutionSample)

Samples: 67414

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 1.04% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 18 | 0.89% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 19 | 0.88% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 20 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 21 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 22 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 24 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 28 | 0.68% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 29 | 0.66% | `java.lang.Enum.ordinal() line: 128` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 33 | 0.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 36 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 37 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 38 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 39 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 43 | 0.44% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 46 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 47 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.41% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 50 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 51 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 52 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 53 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 54 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 55 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 56 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 58 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 59 | 0.35% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 60 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.29% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.26% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 8 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 1.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.34% | `java.util.HashMap.containsKey(Object) line: 602` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 1.04% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 18 | 1.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.84% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 25 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.64% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 29 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


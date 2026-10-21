# JVM hotspots (JFR ExecutionSample)

Samples: 73389

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.94% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 14 | 1.08% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 15 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 18 | 0.93% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.86% | `java.util.HashMap.getNode(Object) line: 585` |
| 23 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 26 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 27 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 29 | 0.67% | `java.lang.Enum.ordinal() line: 128` |
| 30 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 31 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.66% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 34 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 35 | 0.58% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 38 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 40 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 41 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 43 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 44 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 53 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 54 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 55 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 56 | 0.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 57 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 60 | 0.30% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.28% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 5 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 1.94% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 8 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 1.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 12 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.52% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 14 | 1.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.47% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.00% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 19 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 20 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 22 | 0.86% | `java.util.HashMap.containsKey(Object) line: 602` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.66% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 28 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |

GC pauses: 0, total 0 ms


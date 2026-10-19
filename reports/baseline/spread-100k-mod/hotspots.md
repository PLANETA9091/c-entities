# JVM hotspots (JFR ExecutionSample)

Samples: 37359

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 4 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.68% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 7 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.18% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 12 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.04% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 16 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 18 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.77% | `java.util.HashMap.getNode(Object) line: 585` |
| 20 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 23 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 29 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 30 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 31 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 32 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 34 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 35 | 0.50% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.50% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 38 | 0.49% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 39 | 0.47% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 40 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 41 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 43 | 0.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 44 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 45 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 46 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 51 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 52 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 53 | 0.33% | `net.minecraft.world.entity.Mob.checkDespawn() line: 741` |
| 54 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 57 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 58 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 59 | 0.29% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.41% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 4 | 2.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.19% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 14 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 16 | 1.05% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 20 | 0.80% | `java.util.HashMap.containsKey(Object) line: 602` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.64% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 24 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 25 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.55% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 29 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 30 | 0.51% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 36134

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 5 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.93% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 0.90% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 13 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.74% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.66% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 20 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.64% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 24 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 28 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 34 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 36 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.40% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 38 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.36% | `java.lang.String.hashCode() line: 2430` |
| 40 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 46 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 49 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 50 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.27% | `java.util.Optional.isPresent() line: 154` |
| 52 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 60 | 0.24% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.90% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.86% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.78% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.66% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 18 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 19 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 25 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 27 | 0.49% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 28 | 0.48% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 32706

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.68% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 8 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.31% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.91% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.81% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 21 | 0.77% | `java.lang.String.hashCode() line: 2430` |
| 22 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.58% | `java.lang.String.hashCode() line: 2433` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.57% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 37 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 39 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.42% | `java.util.Optional.isPresent() line: 154` |
| 41 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.39% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 44 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 49 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 50 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 51 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 53 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.27% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 59 | 0.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.68% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 6 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.31% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.05% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 16 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 18 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 19 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.59% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.55% | `java.util.HashMap.containsKey(Object) line: 603` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 25117

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.34% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 1.13% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.94% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.78% | `java.util.Optional.isPresent() line: 154` |
| 26 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.60% | `java.util.ArrayDeque.size() line: 647` |
| 33 | 0.57% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 35 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 39 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.47% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 41 | 0.47% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.42% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.41% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 48 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.39% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 53 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 55 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.34% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 10 | 1.30% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 15 | 1.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 16 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.86% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.82% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 23 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.70% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 30 | 0.66% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |

GC pauses: 0, total 0 ms


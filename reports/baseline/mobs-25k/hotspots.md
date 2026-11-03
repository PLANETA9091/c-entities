# JVM hotspots (JFR ExecutionSample)

Samples: 32217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.39% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.11% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 16 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.73% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.68% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 27 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.53% | `java.lang.String.hashCode() line: 2430` |
| 31 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.49% | `java.util.Optional.isPresent() line: 154` |
| 34 | 0.49% | `java.util.ArrayDeque.size() line: 647` |
| 35 | 0.47% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 39 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 41 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.37% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 44 | 0.37% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 45 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 50 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 54 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 55 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.39% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 7 | 1.11% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 1.06% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 19 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.70% | `java.util.HashMap.containsKey(Object) line: 603` |
| 21 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 24 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.53% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

GC pauses: 0, total 0 ms


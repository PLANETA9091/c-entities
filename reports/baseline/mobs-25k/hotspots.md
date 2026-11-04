# JVM hotspots (JFR ExecutionSample)

Samples: 32181

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 1.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.58% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.45% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.03% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 15 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.79% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.78% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 21 | 0.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.71% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.69% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.67% | `java.lang.String.hashCode() line: 2430` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 33 | 0.55% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 34 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.46% | `java.util.Optional.isPresent() line: 154` |
| 39 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 41 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.39% | `java.util.ArrayDeque.size() line: 647` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 48 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.35% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 51 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 58 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 59 | 0.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.45% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.17% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.94% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 13 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.79% | `java.util.HashMap.containsKey(Object) line: 603` |
| 18 | 0.78% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 20 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.71% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.71% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 30 | 0.58% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms


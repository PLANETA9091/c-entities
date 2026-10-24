# JVM hotspots (JFR ExecutionSample)

Samples: 27379

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 8 | 1.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.43% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.29% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 16 | 1.14% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 18 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.96% | `java.lang.String.hashCode() line: 2430` |
| 20 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 23 | 0.76% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 27 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.60% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.55% | `java.util.ArrayDeque.size() line: 647` |
| 33 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.53% | `java.util.Optional.get() line: 142` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 37 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.50% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 39 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.48% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 43 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 46 | 0.42% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 47 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 53 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.25% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.43% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.22% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 8 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 1.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.98% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.96% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 19 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.76% | `java.util.HashMap.containsKey(Object) line: 603` |
| 24 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 27 | 0.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 29 | 0.69% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms


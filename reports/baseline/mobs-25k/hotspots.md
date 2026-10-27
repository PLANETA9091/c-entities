# JVM hotspots (JFR ExecutionSample)

Samples: 27510

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.52% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.97% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.76% | `java.lang.String.hashCode() line: 2430` |
| 25 | 0.68% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 26 | 0.65% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 30 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.61% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.60% | `java.lang.String.hashCode() line: 2433` |
| 34 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.56% | `java.util.ArrayDeque.size() line: 647` |
| 37 | 0.55% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 38 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 40 | 0.51% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 41 | 0.50% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 42 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 47 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 51 | 0.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 52 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 54 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.37% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 57 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 58 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.52% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 8 | 1.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 1.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 1.03% | `java.util.HashMap.containsKey(Object) line: 603` |
| 17 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 20 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 23 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.66% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms


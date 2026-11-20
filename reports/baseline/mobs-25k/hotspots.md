# JVM hotspots (JFR ExecutionSample)

Samples: 33118

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.96% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 0.86% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 16 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.82% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.81% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.71% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.52% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 29 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.49% | `java.lang.String.hashCode() line: 2430` |
| 33 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.43% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 37 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.40% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 39 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.37% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 45 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 50 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 51 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.24% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 53 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 57 | 0.22% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.21% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 60 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 11 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.85% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 14 | 0.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 15 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.79% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 17 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 19 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 22 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 24 | 0.59% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.52% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 28 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms


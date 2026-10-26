# JVM hotspots (JFR ExecutionSample)

Samples: 27207

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.96% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 7 | 1.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.67% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 1.10% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 1.10% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 1.06% | `java.lang.String.hashCode() line: 2430` |
| 20 | 0.96% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 25 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.60% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 33 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 35 | 0.54% | `java.util.Optional.get() line: 142` |
| 36 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.52% | `java.util.ArrayDeque.size() line: 647` |
| 38 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.49% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 45 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 47 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.34% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 51 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.27% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.96% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.67% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.21% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 1.09% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 14 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.68% | `java.util.HashMap.containsKey(Object) line: 603` |
| 23 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 26 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.65% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

GC pauses: 0, total 0 ms


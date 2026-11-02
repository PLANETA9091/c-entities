# JVM hotspots (JFR ExecutionSample)

Samples: 30641

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.60% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.98% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 15 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.79% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.71% | `java.lang.String.hashCode() line: 2430` |
| 24 | 0.71% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.64% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 28 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 32 | 0.53% | `java.util.Optional.isPresent() line: 154` |
| 33 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 34 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.47% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 37 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 46 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 47 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 53 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.26% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 60 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.60% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.50% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 1.05% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 16 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 19 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.74% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.65% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 29 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 30 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms


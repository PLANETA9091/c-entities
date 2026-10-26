# JVM hotspots (JFR ExecutionSample)

Samples: 31254

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.06% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.95% | `java.util.HashMap.getNode(Object) line: 586` |
| 16 | 0.95% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 0.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.80% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 23 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 24 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.55% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 34 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.45% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 38 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.38% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.31% | `java.util.Optional.isPresent() line: 154` |
| 51 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 58 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.27% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 8.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.66% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.06% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.94% | `java.util.HashMap.containsKey(Object) line: 603` |
| 12 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.77% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.69% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 20 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 23 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 29 | 0.56% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms


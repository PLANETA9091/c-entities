# JVM hotspots (JFR ExecutionSample)

Samples: 27778

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 8 | 1.75% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.45% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.11% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 1.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 23 | 0.75% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.72% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 26 | 0.71% | `java.lang.String.hashCode() line: 2430` |
| 27 | 0.70% | `java.util.HashMap.getNode(Object) line: 586` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 29 | 0.67% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 32 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.58% | `java.util.Optional.isPresent() line: 154` |
| 35 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 37 | 0.54% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.51% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 41 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.46% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 47 | 0.37% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 48 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 53 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 54 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 57 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.25% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.75% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.60% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 1.22% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 14 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 1.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 16 | 1.00% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 21 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 25 | 0.72% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 26 | 0.72% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.71% | `java.util.HashMap.containsKey(Object) line: 603` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms


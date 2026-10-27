# JVM hotspots (JFR ExecutionSample)

Samples: 27392

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.38% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.29% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 1.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 1.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 17 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.84% | `java.lang.String.hashCode() line: 2430` |
| 22 | 0.82% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.66% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.61% | `java.util.ArrayDeque.size() line: 647` |
| 34 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.58% | `java.util.HashMap.getNode(Object) line: 586` |
| 38 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.56% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 40 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.53% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 43 | 0.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 44 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.43% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 48 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 51 | 0.37% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 52 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.30% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 58 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 59 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.38% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.31% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 1.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 1.05% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 26 | 0.71% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.70% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 29 | 0.69% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms


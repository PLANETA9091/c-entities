# JVM hotspots (JFR ExecutionSample)

Samples: 30966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.34% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 1.09% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.98% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.88% | `java.lang.String.hashCode() line: 2430` |
| 18 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 23 | 0.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.65% | `java.lang.String.hashCode() line: 2433` |
| 26 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 29 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 31 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.53% | `java.util.Optional.get() line: 142` |
| 36 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 38 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 41 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 43 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 44 | 0.40% | `java.util.ArrayDeque.size() line: 647` |
| 45 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 47 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.32% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 54 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 55 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 59 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.34% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.10% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 19 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.65% | `java.util.HashMap.containsKey(Object) line: 603` |
| 25 | 0.65% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 29 | 0.62% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.61% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |

GC pauses: 0, total 0 ms


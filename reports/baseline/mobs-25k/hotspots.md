# JVM hotspots (JFR ExecutionSample)

Samples: 35143

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.75% | `java.util.HashMap.getNode(Object) line: 586` |
| 3 | 3.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 10 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.09% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 14 | 0.99% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.90% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 17 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.64% | `java.lang.String.hashCode() line: 2430` |
| 26 | 0.63% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.52% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 34 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.45% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 39 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.43% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 43 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 44 | 0.34% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 48 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 50 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 51 | 0.27% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.23% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 59 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.39% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 3.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.06% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.98% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 13 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 14 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 19 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 21 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.63% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.61% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 29 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms


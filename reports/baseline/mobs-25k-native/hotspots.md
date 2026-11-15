# JVM hotspots (JFR ExecutionSample)

Samples: 33528

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.05% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 6 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.15% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 18 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.66% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.63% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 28 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 30 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 34 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.44% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.42% | `java.util.ArrayDeque.size() line: 647` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 42 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 49 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.35% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 52 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 55 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 56 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 58 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 59 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.22% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 6 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.15% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 1.11% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 1.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 0.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.68% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 24 | 0.67% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 26 | 0.61% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 28 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.52% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms


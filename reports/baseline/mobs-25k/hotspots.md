# JVM hotspots (JFR ExecutionSample)

Samples: 34616

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 10 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.19% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.02% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 15 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.81% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 28 | 0.58% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.54% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 31 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.50% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.49% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 35 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 36 | 0.47% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 37 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.41% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 43 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 46 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.34% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 49 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 52 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 53 | 0.32% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 54 | 0.30% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 55 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 58 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.23% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.29% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 1.03% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 9 | 0.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 11 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 17 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.68% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 24 | 0.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.58% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 27 | 0.56% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 28 | 0.54% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 29 | 0.54% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 30 | 0.54% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |

GC pauses: 0, total 0 ms


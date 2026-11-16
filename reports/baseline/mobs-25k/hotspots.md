# JVM hotspots (JFR ExecutionSample)

Samples: 35809

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.05% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 7 | 1.92% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.76% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 20 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.58% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.58% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 28 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 35 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 38 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 42 | 0.41% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 44 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 46 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.36% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 48 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 50 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.32% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.32% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 54 | 0.30% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 55 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 58 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 60 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 6 | 1.63% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 12 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 14 | 0.82% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 15 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 20 | 0.65% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 22 | 0.61% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.58% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.55% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.54% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 29 | 0.51% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 30 | 0.50% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |

GC pauses: 0, total 0 ms


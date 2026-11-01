# JVM hotspots (JFR ExecutionSample)

Samples: 31351

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.40% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 13 | 1.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.96% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.86% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.69% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.64% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.59% | `java.lang.String.hashCode() line: 2430` |
| 29 | 0.57% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 30 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 31 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 35 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.38% | `java.util.ArrayDeque.size() line: 647` |
| 45 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 49 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.28% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 57 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 58 | 0.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 59 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 60 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.40% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.09% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 1.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 1.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 14 | 0.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 16 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 17 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 22 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.67% | `java.util.HashMap.containsKey(Object) line: 603` |
| 24 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.64% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.64% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.57% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms


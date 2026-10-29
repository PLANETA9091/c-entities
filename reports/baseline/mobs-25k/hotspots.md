# JVM hotspots (JFR ExecutionSample)

Samples: 35292

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.26% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 14 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 17 | 0.85% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.64% | `java.lang.String.hashCode() line: 2430` |
| 25 | 0.64% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 34 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 36 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 37 | 0.47% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 38 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 39 | 0.43% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 42 | 0.41% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 46 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 50 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.32% | `java.util.Optional.isPresent() line: 154` |
| 53 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 55 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.29% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 58 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 59 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.37% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.26% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 11 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.01% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 16 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.80% | `java.util.HashMap.containsKey(Object) line: 603` |
| 20 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.64% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 25 | 0.61% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |

GC pauses: 0, total 0 ms


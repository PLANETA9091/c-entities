# JVM hotspots (JFR ExecutionSample)

Samples: 43241

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.28% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.04% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.95% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.72% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 18 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 24 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.55% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 27 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 28 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.51% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.41% | `java.util.ArrayDeque.size() line: 647` |
| 40 | 0.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 41 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 44 | 0.35% | `java.util.Optional.get() line: 142` |
| 45 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.32% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 53 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 54 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 55 | 0.27% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 57 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.25% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.25% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 8.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.28% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 9 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.03% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 1.00% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 19 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.65% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 21 | 0.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 23 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 27 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.52% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.52% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |

GC pauses: 0, total 0 ms


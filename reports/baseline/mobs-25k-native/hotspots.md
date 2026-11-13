# JVM hotspots (JFR ExecutionSample)

Samples: 54584

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.73% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.71% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.56% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.52% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 22 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.46% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 26 | 0.44% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 27 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 28 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 35 | 0.38% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 36 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 37 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 44 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.31% | `java.lang.String.hashCode() line: 2430` |
| 46 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 47 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 49 | 0.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 50 | 0.28% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 51 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 52 | 0.26% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.26% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 54 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.22% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 57 | 0.21% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 58 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 60 | 0.20% | `java.util.ImmutableCollections$SetN.probe(Object) line: 1258` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 0.94% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.72% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 12 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 15 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getFluidState(int, int, int) line: 355` |
| 17 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 18 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 19 | 0.55% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 20 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.49% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 22 | 0.48% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.47% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 24 | 0.46% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 25 | 0.46% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.41% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |

GC pauses: 0, total 0 ms


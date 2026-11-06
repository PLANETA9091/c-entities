# JVM hotspots (JFR ExecutionSample)

Samples: 39969

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.46% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.22% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.13% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.66% | `java.util.HashMap.getNode(Object) line: 586` |
| 17 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.51% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 24 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.45% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 29 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.43% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 33 | 0.42% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 38 | 0.38% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 39 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.27% | `java.util.Optional.isPresent() line: 154` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 52 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 53 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 54 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.22% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.21% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.13% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 10 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.66% | `java.util.HashMap.containsKey(Object) line: 603` |
| 14 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 17 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.53% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.52% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 24 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 25 | 0.50% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.48% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 28 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 57397

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 0.89% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 9 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.46% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 21 | 0.45% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.40% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 27 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 29 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 31 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.31% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 35 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.28% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 44 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 45 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 46 | 0.26% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.26% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 48 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.24% | `java.util.ArrayDeque.size() line: 647` |
| 51 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 53 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 57 | 0.20% | `java.lang.String.hashCode() line: 2430` |
| 58 | 0.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 541` |
| 60 | 0.18% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 41.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.89% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 6 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 9 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 10 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 0.61% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.45% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 20 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.45% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 24 | 0.40% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 26 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.37% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.35% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms


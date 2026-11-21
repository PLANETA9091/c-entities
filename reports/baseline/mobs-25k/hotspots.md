# JVM hotspots (JFR ExecutionSample)

Samples: 49801

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.13% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 0.58% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 16 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.52% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 19 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 21 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.47% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 24 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 27 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 28 | 0.43% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 29 | 0.42% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.41% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 31 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 33 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 38 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 40 | 0.33% | `java.util.ArrayDeque.size() line: 647` |
| 41 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 45 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 46 | 0.31% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 50 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 51 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.25% | `java.lang.String.hashCode() line: 2430` |
| 54 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.22% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 56 | 0.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.18% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.18% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 60 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.84% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.73% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 13 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 0.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 16 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.57% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 18 | 0.50% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 19 | 0.48% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 20 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 21 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 22 | 0.47% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 26 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 29 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |

GC pauses: 0, total 0 ms


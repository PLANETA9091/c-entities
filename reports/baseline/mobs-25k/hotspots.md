# JVM hotspots (JFR ExecutionSample)

Samples: 56995

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.13% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 8 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 17 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.42% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 23 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 24 | 0.41% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 25 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 30 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 34 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.31% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 44 | 0.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 45 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 46 | 0.28% | `java.lang.String.hashCode() line: 2430` |
| 47 | 0.27% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 48 | 0.27% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 49 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.26% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 51 | 0.26% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 53 | 0.25% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 54 | 0.24% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 56 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 59 | 0.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.lambda$forEachModifier$2(EquipmentSlot, BiConsumer, Holder, int) line: 405` |
| 60 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.11% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 6 | 0.89% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 9 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 0.63% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 11 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 16 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 0.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.47% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 21 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 22 | 0.44% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.41% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.41% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 27 | 0.38% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 29 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms


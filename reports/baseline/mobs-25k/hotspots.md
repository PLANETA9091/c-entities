# JVM hotspots (JFR ExecutionSample)

Samples: 57845

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.10% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.94% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 8 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 0.73% | `com.google.common.collect.RegularImmutableMap.get(Object, ImmutableMapEntry[], int) line: 282` |
| 12 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.38% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 25 | 0.38% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 26 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 29 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 32 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 34 | 0.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 35 | 0.32% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.31% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 39 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 41 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 44 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 47 | 0.24% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 51 | 0.22% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.21% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 53 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.20% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 57 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.19% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.13% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.15% | `net.minecraft.world.entity.Mob.tick() line: 410` |
| 8 | 0.81% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 9 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 13 | 0.59% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 14 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.48% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 21 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 23 | 0.40% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.39% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.39% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 26 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.36% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 29 | 0.35% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms


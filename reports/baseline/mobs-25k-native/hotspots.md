# JVM hotspots (JFR ExecutionSample)

Samples: 51252

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.08% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.95% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.89% | `com.google.common.collect.RegularImmutableMap.get(Object, ImmutableMapEntry[], int) line: 282` |
| 12 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.42% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 23 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 27 | 0.38% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 28 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 29 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 31 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 37 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.31% | `java.lang.String.hashCode() line: 2437` |
| 39 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.30% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 41 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 44 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.24% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 50 | 0.24% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 51 | 0.23% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.22% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.22% | `java.lang.String.hashCode() line: 2430` |
| 56 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 57 | 0.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 58 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 60 | 0.19% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 1.87% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.93% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 9 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 13 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.60% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.42% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.40% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 22 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.39% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 24 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 25 | 0.38% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 26 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.36% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.36% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.34% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |

GC pauses: 0, total 0 ms


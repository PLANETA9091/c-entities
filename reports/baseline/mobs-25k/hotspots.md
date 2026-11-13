# JVM hotspots (JFR ExecutionSample)

Samples: 50339

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 4 | 2.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.58% | `com.google.common.collect.RegularImmutableMap.get(Object, ImmutableMapEntry[], int) line: 282` |
| 17 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.49% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 23 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 25 | 0.45% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 26 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 27 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 29 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.41% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 34 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 35 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 39 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 42 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 47 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 48 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.28% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 50 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 52 | 0.26% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.22% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 56 | 0.20% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 57 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.19% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 59 | 0.18% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.17% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.54% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 13 | 0.65% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |
| 14 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 17 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.54% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 19 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.52% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.49% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 25 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.42% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 29 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 50538

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 4 | 1.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 16 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 24 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.46% | `java.lang.String.hashCode() line: 2430` |
| 26 | 0.45% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 27 | 0.45% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 28 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.40% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 35 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 38 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 40 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.34% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 44 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 46 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.30% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 50 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 52 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 53 | 0.27% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 54 | 0.24% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 55 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.33% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 12 | 0.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.65% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 18 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 19 | 0.50% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 20 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 21 | 0.48% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 22 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.46% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 26 | 0.46% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 27 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 29 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 30 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms


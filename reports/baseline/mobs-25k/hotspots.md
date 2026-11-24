# JVM hotspots (JFR ExecutionSample)

Samples: 36436

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.84% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.75% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 22 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 24 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.42% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 35 | 0.40% | `java.util.ArrayDeque.size() line: 647` |
| 36 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 38 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.39% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 41 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 44 | 0.32% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 45 | 0.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 47 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.26% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2902` |
| 52 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.23% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.20% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5027` |
| 59 | 0.20% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.20% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 5 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.53% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 8 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.01% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 14 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 16 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.69% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 24 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 25 | 0.52% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.52% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 27 | 0.50% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 28 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3793` |
| 29 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3597` |
| 30 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms


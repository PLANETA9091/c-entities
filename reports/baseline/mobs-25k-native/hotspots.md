# JVM hotspots (JFR ExecutionSample)

Samples: 31403

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.74% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 3.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.16% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 2.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.08% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.89% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.75% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 20 | 0.75% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 23 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 24 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 30 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.51% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 33 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.41% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 37 | 0.40% | `java.util.ArrayDeque.size() line: 647` |
| 38 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 39 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.39% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.34% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 43 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 45 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 49 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 51 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 54 | 0.26% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 55 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.22% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 59 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.80% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 12 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 14 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 0.73% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 18 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.69% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 20 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 21 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 24 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.55% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.54% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 28 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |

GC pauses: 0, total 0 ms


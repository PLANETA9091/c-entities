# JVM hotspots (JFR ExecutionSample)

Samples: 35984

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.51% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 3.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 5 | 2.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.92% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 9 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.78% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 18 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 23 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 27 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.54% | `java.lang.String.hashCode() line: 2430` |
| 29 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 30 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 34 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 36 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.39% | `java.util.ArrayDeque.size() line: 647` |
| 39 | 0.39% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 40 | 0.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 41 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.34% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 43 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.32% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 46 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 48 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.27% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.26% | `java.util.HashMap.getNode(Object) line: 588` |
| 51 | 0.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 58 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 59 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.19% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 3.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Entity, AABB, List, Predicate) line: 598` |
| 6 | 2.69% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 9 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 15 | 0.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 16 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.70% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 20 | 0.70% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 21 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.63% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 25 | 0.55% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.52% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 29 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms


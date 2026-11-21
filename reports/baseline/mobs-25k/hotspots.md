# JVM hotspots (JFR ExecutionSample)

Samples: 39542

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.09% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.68% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 18 | 0.67% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 19 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.65% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 21 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 25 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.51% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 30 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 34 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 36 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.38% | `java.lang.String.hashCode() line: 2430` |
| 39 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 44 | 0.33% | `java.util.ArrayDeque.size() line: 647` |
| 45 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 52 | 0.30% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.26% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 57 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.23% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 1.09% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 12 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 13 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.69% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 0.68% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 18 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.62% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.56% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.55% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 22 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 23 | 0.52% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 25 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 28 | 0.47% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.43% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


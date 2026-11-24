# JVM hotspots (JFR ExecutionSample)

Samples: 36498

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.15% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 6 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.50% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.72% | `java.lang.String.hashCode() line: 2430` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 28 | 0.54% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 29 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 40 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 41 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.lambda$forEachModifier$2(EquipmentSlot, BiConsumer, Holder, int) line: 405` |
| 42 | 0.38% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 44 | 0.36% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 45 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 47 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 48 | 0.32% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 49 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 50 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 53 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 6 | 1.29% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.24% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 11 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |
| 14 | 0.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 15 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 18 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3793` |
| 22 | 0.62% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.61% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 24 | 0.61% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 25 | 0.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 26 | 0.58% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 27 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.54% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |

GC pauses: 0, total 0 ms


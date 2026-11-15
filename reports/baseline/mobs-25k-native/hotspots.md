# JVM hotspots (JFR ExecutionSample)

Samples: 29637

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 19 | 0.74% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.72% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.64% | `java.lang.String.hashCode() line: 2430` |
| 23 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 28 | 0.56% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.44% | `java.util.ArrayDeque.size() line: 647` |
| 34 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.42% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 37 | 0.41% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 38 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 39 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 40 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.39% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 42 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 43 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 44 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 45 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 48 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.30% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 50 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 53 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.23% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 57 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.22% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 60 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.18% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 13 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.76% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 15 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.71% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 17 | 0.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 18 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.66% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 24 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 28 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.52% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms


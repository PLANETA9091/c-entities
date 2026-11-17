# JVM hotspots (JFR ExecutionSample)

Samples: 41486

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 3.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 22 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 26 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.44% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 37 | 0.36% | `java.lang.String.hashCode() line: 2430` |
| 38 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.34% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 42 | 0.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 43 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 45 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 46 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.26% | `java.util.ArrayDeque.size() line: 647` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 54 | 0.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 56 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.21% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.20% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.20% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.19% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.04% | `java.util.HashMap.containsKey(Object) line: 603` |
| 7 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 9 | 0.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.72% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 11 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 16 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 17 | 0.54% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 18 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.53% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.50% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 22 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 24 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.44% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 26 | 0.44% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 27 | 0.43% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.42% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |

GC pauses: 0, total 0 ms


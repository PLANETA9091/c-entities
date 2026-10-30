# JVM hotspots (JFR ExecutionSample)

Samples: 35068

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.25% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 0.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.79% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.78% | `java.lang.String.hashCode() line: 2430` |
| 21 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.75% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 23 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.66% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.62% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 36 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.43% | `java.util.Optional.isPresent() line: 154` |
| 39 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 41 | 0.41% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.36% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.32% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 51 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 53 | 0.28% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 54 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.00% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.25% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.08% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.74% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 20 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.61% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.56% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |

GC pauses: 0, total 0 ms


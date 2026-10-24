# JVM hotspots (JFR ExecutionSample)

Samples: 35917

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 3.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.21% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.85% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 16 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 18 | 0.77% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 19 | 0.74% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 24 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.59% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 27 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.52% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 29 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 30 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.43% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 35 | 0.43% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 36 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.41% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 38 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 39 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 41 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.36% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 46 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.34% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 54 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 56 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.25% | `java.util.Optional.get() line: 142` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 8.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.63% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.47% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 7 | 1.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.21% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 0.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 0.77% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.77% | `java.util.HashMap.containsKey(Object) line: 603` |
| 17 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 18 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 20 | 0.59% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 21 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.53% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 23 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.50% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 25 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 26 | 0.49% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.48% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 256` |
| 28 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 30 | 0.47% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


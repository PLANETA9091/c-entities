# JVM hotspots (JFR ExecutionSample)

Samples: 48333

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.36% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.17% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 16 | 0.70% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 17 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.62% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.58% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 24 | 0.53% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.52% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 26 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 28 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.46% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 35 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.40% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 46 | 0.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 47 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 50 | 0.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 51 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.29% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 60 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.32% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.19% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 7 | 1.17% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.70% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 17 | 0.68% | `java.util.HashMap.containsKey(Object) line: 603` |
| 18 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.58% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 22 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.52% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.52% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 25 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.50% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.49% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.47% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 30 | 0.46% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms


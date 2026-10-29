# JVM hotspots (JFR ExecutionSample)

Samples: 52612

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.87% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.68% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 14 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.50% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 23 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.45% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 27 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.38% | `java.lang.String.hashCode() line: 2430` |
| 29 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.36% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 32 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 33 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 34 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 39 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 40 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 43 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.28% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 48 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.27% | `java.util.ArrayDeque.size() line: 647` |
| 50 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.26% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.23% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.23% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 57 | 0.23% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 60 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 8 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 0.80% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.68% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 13 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 20 | 0.50% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 21 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 26 | 0.43% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms


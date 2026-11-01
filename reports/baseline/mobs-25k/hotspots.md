# JVM hotspots (JFR ExecutionSample)

Samples: 52160

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.76% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 13 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.75% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 16 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.54% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 31 | 0.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 32 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.31% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 36 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.31% | `java.lang.String.hashCode() line: 2430` |
| 38 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.29% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 42 | 0.29% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 43 | 0.28% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 44 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 46 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.25% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 48 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 49 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 51 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 52 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.22% | `java.util.Optional.isPresent() line: 154` |
| 54 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.20% | `java.util.ArrayDeque.size() line: 647` |
| 59 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.19% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.85% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 0.76% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 0.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.58% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 0.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 17 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 22 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.40% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.40% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 27 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.37% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms


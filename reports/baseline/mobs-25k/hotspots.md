# JVM hotspots (JFR ExecutionSample)

Samples: 39131

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.08% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.83% | `java.util.HashMap.getNode(Object) line: 586` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 17 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.75% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 19 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.62% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 31 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.39% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.39% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 39 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 40 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 42 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.32% | `java.util.Optional.isPresent() line: 154` |
| 44 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 45 | 0.32% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.30% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 50 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 51 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.23% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 59 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 7.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.30% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.08% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.06% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.81% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.75% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 16 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 19 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 21 | 0.55% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 24 | 0.50% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.50% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.49% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 28 | 0.48% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 29 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 54102

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.85% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.69% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 11 | 0.69% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 20 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.47% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 27 | 0.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 28 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.41% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 32 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 34 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 39 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 46 | 0.30% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 47 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 50 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.24% | `java.lang.String.hashCode() line: 2430` |
| 58 | 0.23% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 60 | 0.22% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.85% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 0.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.69% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 0.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.66% | `java.util.HashMap.containsKey(Object) line: 603` |
| 12 | 0.65% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 21 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.41% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 29 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 30 | 0.39% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 52811

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.03% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.70% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.45% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 24 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 25 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.32% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 45 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.27% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 48 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 51 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.24% | `java.lang.Double.isFinite(double) line: 1023` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 55 | 0.22% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 56 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.21% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 58 | 0.20% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.20% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 60 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.03% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 0.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.62% | `java.util.HashMap.containsKey(Object) line: 603` |
| 12 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 0.55% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 18 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.45% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 22 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 24 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 27 | 0.41% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 30 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos) line: 863` |

GC pauses: 0, total 0 ms


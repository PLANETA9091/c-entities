# JVM hotspots (JFR ExecutionSample)

Samples: 52234

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.80% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.76% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.74% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.50% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 25 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 29 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.37% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 34 | 0.36% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 35 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 37 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 40 | 0.33% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 41 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 44 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 47 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 52 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 53 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 55 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 58 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 59 | 0.21% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.80% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.73% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.58% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.50% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 20 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 25 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 26 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.42% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.41% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |

GC pauses: 0, total 0 ms


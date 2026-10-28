# JVM hotspots (JFR ExecutionSample)

Samples: 48644

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.01% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.14% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 7 | 1.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.73% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 17 | 0.69% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 18 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 26 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 32 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 36 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.31% | `java.lang.String.hashCode() line: 2430` |
| 40 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 44 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.28% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 47 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 48 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.24% | `java.lang.Double.isFinite(double) line: 1023` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 52 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 54 | 0.22% | `java.util.ImmutableCollections$SetN.probe(Object) line: 1258` |
| 55 | 0.21% | `java.lang.ThreadLocal.get() line: 171` |
| 56 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.21% | `java.util.Optional.isPresent() line: 154` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.14% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 6 | 1.13% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 0.76% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 10 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 0.69% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 13 | 0.68% | `java.util.HashMap.containsKey(Object) line: 603` |
| 14 | 0.61% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.42% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 24 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 27 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos) line: 863` |
| 28 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |

GC pauses: 0, total 0 ms


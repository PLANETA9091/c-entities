# JVM hotspots (JFR ExecutionSample)

Samples: 33437

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.55% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.97% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 9 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.93% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 12 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 15 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 16 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 17 | 0.79% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 18 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 20 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.74% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 22 | 0.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 23 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 24 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.61% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.58% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 31 | 0.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 32 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1410` |
| 33 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 35 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 37 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 38 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 40 | 0.47% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 41 | 0.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 42 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.setXRot(float) line: 5100` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 48 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 49 | 0.33% | `java.util.ArrayDeque.size() line: 648` |
| 50 | 0.33% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.33% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 54 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 55 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 57 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 58 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 60 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.56% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.97% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 5 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.98% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 9 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 11 | 0.86% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 13 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 14 | 0.85% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 15 | 0.80% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 17 | 0.71% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.68% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 238` |
| 23 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 24 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 26 | 0.61% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 27 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 30 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 35944

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.75% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 5.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.33% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 2.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.26% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 10 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 14 | 0.95% | `java.util.EnumMap.get(Object) line: 247` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 17 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.85% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 20 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 24 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 25 | 0.81% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 26 | 0.79% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 28 | 0.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 31 | 0.67% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 34 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 35 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 36 | 0.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 37 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 38 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 39 | 0.55% | `java.util.ArrayDeque.size() line: 648` |
| 40 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |
| 41 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 43 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 45 | 0.46% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 46 | 0.42% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.41% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 48 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 49 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 50 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 51 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 53 | 0.36% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 55 | 0.34% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 56 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 58 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 59 | 0.29% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.75% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.33% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 8 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 9 | 1.15% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.09% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 12 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 13 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.88% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 18 | 0.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 19 | 0.83% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 20 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 22 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 25 | 0.77% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 27 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 29 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 30 | 0.69% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


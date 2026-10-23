# JVM hotspots (JFR ExecutionSample)

Samples: 31354

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.21% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 3 | 4.93% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 4.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.49% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 1.02% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 1.02% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.92% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 15 | 0.89% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 20 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.80% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 22 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 25 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 27 | 0.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 31 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 32 | 0.60% | `java.util.ArrayDeque.size() line: 648` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 35 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 36 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 37 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.47% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 40 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 41 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 42 | 0.42% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 43 | 0.40% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 46 | 0.38% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.37% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 48 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 50 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 51 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 53 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 147` |
| 54 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 56 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.27% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 58 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 59 | 0.25% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 60 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.22% | `net.minecraft.world.entity.Mob.serverAiStep() line: 788` |
| 3 | 7.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.95% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 2.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 2.49% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 1.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 10 | 1.10% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 1.04% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 14 | 0.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 0.92% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 16 | 0.87% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 18 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 19 | 0.85% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 20 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 21 | 0.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 23 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 24 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 25 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 27 | 0.67% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 29 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.60% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


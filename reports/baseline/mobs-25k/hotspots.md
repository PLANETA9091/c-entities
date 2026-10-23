# JVM hotspots (JFR ExecutionSample)

Samples: 36071

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.94% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 4.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.71% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.19% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.03% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 16 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 19 | 0.90% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 22 | 0.83% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 27 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 28 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 31 | 0.66% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 33 | 0.61% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 34 | 0.60% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 35 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.58% | `java.util.ArrayDeque.size() line: 648` |
| 37 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 38 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 40 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.40% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 45 | 0.40% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 46 | 0.40% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 47 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 48 | 0.38% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 50 | 0.36% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 52 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 53 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 54 | 0.32% | `java.lang.ThreadLocal.get() line: 172` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 56 | 0.32% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 57 | 0.32% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 58 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 59 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 60 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.94% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.72% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 9 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 10 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 1.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 1.03% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 16 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 18 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.90% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 26 | 0.69% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.69% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 28 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 29 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 774` |
| 30 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |

GC pauses: 0, total 0 ms


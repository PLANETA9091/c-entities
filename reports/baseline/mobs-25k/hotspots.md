# JVM hotspots (JFR ExecutionSample)

Samples: 49037

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.64% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.00% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.90% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 13 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 15 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 17 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.66% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.66% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 21 | 0.62% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.59% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 25 | 0.54% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 26 | 0.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.52% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 30 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 31 | 0.48% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 32 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.47% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 34 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 37 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 38 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 39 | 0.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 40 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 42 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 43 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 44 | 0.34% | `java.lang.ThreadLocal.get() line: 172` |
| 45 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 46 | 0.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 47 | 0.31% | `java.util.ArrayDeque.addLast(Object) line: 304` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.setXRot(float) line: 5100` |
| 49 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 50 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.28% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 52 | 0.28% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 53 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 54 | 0.26% | `java.util.ArrayDeque.size() line: 648` |
| 55 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 56 | 0.25% | `java.util.concurrent.atomic.AtomicIntegerFieldUpdater$AtomicIntegerFieldUpdaterImpl.compareAndSet(Object, int, int) line: 491` |
| 57 | 0.25% | `java.util.concurrent.atomic.Striped64.getProbe() line: 200` |
| 58 | 0.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.65% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.00% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.88% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 9 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 10 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 12 | 0.78% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 15 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 16 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.60% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 19 | 0.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 20 | 0.58% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 21 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 24 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.53% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 238` |
| 26 | 0.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 27 | 0.52% | `java.util.concurrent.atomic.Striped64.casBase(long, long) line: 181` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 30 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 48271

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.55% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.08% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 0.69% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 14 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.63% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.61% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 21 | 0.58% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 23 | 0.57% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 24 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 27 | 0.50% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 28 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.47% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 31 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 33 | 0.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 36 | 0.45% | `java.lang.ThreadLocal.get() line: 172` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 38 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 40 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 42 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 43 | 0.36% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 44 | 0.32% | `net.minecraft.world.entity.Entity.setXRot(float) line: 5100` |
| 45 | 0.32% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 46 | 0.32% | `java.util.ArrayDeque.size() line: 648` |
| 47 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 49 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 50 | 0.30% | `java.util.concurrent.locks.AbstractQueuedSynchronizer.compareAndSetState(int, int) line: 569` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 52 | 0.29% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 53 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 54 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 55 | 0.27% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 56 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 58 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 59 | 0.25% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 60 | 0.24% | `java.util.concurrent.atomic.AtomicIntegerFieldUpdater$AtomicIntegerFieldUpdaterImpl.compareAndSet(Object, int, int) line: 491` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.55% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.17% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.11% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 10 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 11 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.67% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 14 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.66% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 16 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 18 | 0.61% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 20 | 0.57% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 21 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.52% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 27 | 0.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 28 | 0.49% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 238` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

GC pauses: 0, total 0 ms


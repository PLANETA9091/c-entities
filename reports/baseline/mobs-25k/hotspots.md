# JVM hotspots (JFR ExecutionSample)

Samples: 45895

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.73% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.47% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.25% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 0.77% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 13 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 17 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.68% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 19 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 20 | 0.65% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 21 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 23 | 0.55% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 24 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 25 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 27 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 28 | 0.51% | `java.util.EnumMap.get(Object) line: 247` |
| 29 | 0.49% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 31 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 32 | 0.47% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 34 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 35 | 0.46% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 36 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.44% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 38 | 0.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 40 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 42 | 0.36% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 43 | 0.35% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 44 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 45 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 46 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 47 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 49 | 0.30% | `net.minecraft.world.entity.Entity.setXRot(float) line: 5100` |
| 50 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 51 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 53 | 0.29% | `java.util.ArrayDeque.size() line: 648` |
| 54 | 0.29% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 55 | 0.27% | `java.util.concurrent.atomic.Striped64.getProbe() line: 200` |
| 56 | 0.27% | `java.util.concurrent.atomic.AtomicIntegerFieldUpdater$AtomicIntegerFieldUpdaterImpl.compareAndSet(Object, int, int) line: 491` |
| 57 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 59 | 0.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 60 | 0.25% | `java.util.ArrayDeque.addLast(Object) line: 304` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.73% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.47% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 10 | 0.77% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.74% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 12 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 13 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 16 | 0.69% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 17 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 18 | 0.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 19 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.57% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 238` |
| 22 | 0.56% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 23 | 0.55% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 26 | 0.53% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 27 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 28 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 30 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 34670

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 4.45% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.79% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 1.19% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.06% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 17 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.95% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 21 | 0.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 23 | 0.83% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 27 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 31 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.63% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 35 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 38 | 0.51% | `java.util.ArrayDeque.size() line: 648` |
| 39 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 40 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.48% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 42 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 43 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 44 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 46 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 47 | 0.41% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 48 | 0.40% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 49 | 0.39% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 50 | 0.39% | `java.lang.ThreadLocal.get() line: 172` |
| 51 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 52 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.37% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 54 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.34% | `java.util.ArrayDeque.addLast(Object) line: 304` |
| 56 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 57 | 0.32% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 59 | 0.30% | `java.lang.ThreadLocal.set(Object) line: 251` |
| 60 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.44% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.79% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 9 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 10 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 11 | 1.00% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 12 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 13 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 15 | 0.89% | `net.minecraft.world.item.ItemStack.isBroken() line: 609` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 17 | 0.89% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 18 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 19 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 24 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 26 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 27 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 28 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 29 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |

GC pauses: 0, total 0 ms


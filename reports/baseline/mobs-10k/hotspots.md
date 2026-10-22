# JVM hotspots (JFR ExecutionSample)

Samples: 11490

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.60% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 3 | 2.24% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.52% | `java.util.EnumMap.get(Object) line: 247` |
| 7 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 10 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.20% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.08% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 16 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.90% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 20 | 0.90% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 21 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 22 | 0.78% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 23 | 0.76% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 25 | 0.74% | `io.netty.util.internal.shaded.org.jctools.util.UnsafeRefArrayAccess.soRefElement(Object[], long, Object) line: 63` |
| 26 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.68% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 29 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 30 | 0.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 31 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 33 | 0.58% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 35 | 0.57% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 36 | 0.56% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 37 | 0.54% | `java.util.ArrayDeque.size() line: 648` |
| 38 | 0.53% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 39 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 40 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 41 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 42 | 0.47% | `java.lang.ThreadLocal.get() line: 172` |
| 43 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 486` |
| 46 | 0.44% | `java.lang.String.length() line: 1508` |
| 47 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 147` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 50 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 51 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 741` |
| 52 | 0.36% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 53 | 0.35% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 56 | 0.33% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2789` |
| 57 | 0.33% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 58 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 59 | 0.32% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 60 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.60% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 2.25% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 8 | 1.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 9 | 1.31% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 10 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.21% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 12 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 13 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 1.11% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 16 | 1.09% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 20 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 22 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 24 | 0.74% | `io.netty.util.internal.shaded.org.jctools.queues.BaseMpscLinkedArrayQueue.offer(Object) line: 280` |
| 25 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 27 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 771` |
| 28 | 0.68% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 29 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |

GC pauses: 0, total 0 ms


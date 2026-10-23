# JVM hotspots (JFR ExecutionSample)

Samples: 54255

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.69% | `java.util.HashMap.getNode(Object) line: 585` |
| 2 | 4.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.22% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 7 | 0.91% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 8 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 13 | 0.65% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 14 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 16 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.61% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 18 | 0.61% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 19 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 20 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 22 | 0.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.48% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 24 | 0.45% | `java.lang.ThreadLocal.get() line: 172` |
| 25 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 29 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 30 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 31 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 32 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 34 | 0.34% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 35 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 36 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 37 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 38 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 39 | 0.32% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 40 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 42 | 0.30% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 43 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 44 | 0.29% | `java.util.ArrayDeque.size() line: 648` |
| 45 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 46 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 47 | 0.28% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 48 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 50 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 51 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 52 | 0.25% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.25% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 54 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.24% | `io.netty.util.internal.InternalThreadLocalMap.get() line: 113` |
| 56 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 57 | 0.23% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 58 | 0.22% | `java.util.concurrent.atomic.Striped64.getProbe() line: 200` |
| 59 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.22% | `java.util.EnumMap.unmaskNull(Object) line: 127` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.68% | `java.util.HashMap.containsKey(Object) line: 602` |
| 2 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 4 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.22% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 9 | 0.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 0.69% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 0.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 13 | 0.65% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 14 | 0.64% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 15 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 17 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 0.61% | `net.minecraft.world.item.ItemStack.isBroken() line: 609` |
| 19 | 0.61% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 20 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.55% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 22 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 23 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 24 | 0.50% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 25 | 0.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 26 | 0.49% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 27 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 29 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |

GC pauses: 0, total 0 ms


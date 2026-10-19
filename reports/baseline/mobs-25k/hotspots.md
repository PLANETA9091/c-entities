# JVM hotspots (JFR ExecutionSample)

Samples: 50316

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 3.47% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.49% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 15 | 0.60% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.59% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 18 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 0.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.54% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 22 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 26 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 29 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.42% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 32 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 33 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 34 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 37 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 38 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 40 | 0.38% | `java.lang.ThreadLocal.get() line: 172` |
| 41 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 42 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 43 | 0.37% | `java.util.ArrayDeque.size() line: 648` |
| 44 | 0.37% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 45 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 46 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 48 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 49 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.32% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 605` |
| 51 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 52 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 53 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 501` |
| 54 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 55 | 0.29% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 56 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 57 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 59 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 60 | 0.25% | `java.util.EnumMap.unmaskNull(Object) line: 127` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 3.46% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.49% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 10 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 11 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 13 | 0.65% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 14 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 17 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 18 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 20 | 0.57% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 21 | 0.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 22 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.55% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 24 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 26 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 27 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 28 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 30 | 0.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |

GC pauses: 0, total 0 ms


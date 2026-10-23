# JVM hotspots (JFR ExecutionSample)

Samples: 54883

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.12% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 15 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.67% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.59% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.58% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 22 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 23 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 24 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.53% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 26 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 30 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 31 | 0.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 35 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 36 | 0.40% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 37 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 38 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 39 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 40 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 41 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 42 | 0.34% | `java.util.ArrayDeque.size() line: 648` |
| 43 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 44 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 46 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.31% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 48 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 49 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 50 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 51 | 0.30% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 52 | 0.29% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 56 | 0.26% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 605` |
| 57 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 500` |
| 58 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 59 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 60 | 0.24% | `io.netty.util.internal.InternalThreadLocalMap.get() line: 113` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.90% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.12% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.83% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 12 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 13 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 14 | 0.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 16 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 20 | 0.60% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 21 | 0.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 22 | 0.56% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 24 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 25 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 29 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.49% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |

GC pauses: 0, total 0 ms


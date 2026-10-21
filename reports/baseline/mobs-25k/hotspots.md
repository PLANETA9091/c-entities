# JVM hotspots (JFR ExecutionSample)

Samples: 53365

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 3.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.80% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.69% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 17 | 0.67% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 20 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 21 | 0.60% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 22 | 0.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 26 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 29 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 31 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 32 | 0.43% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 33 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 34 | 0.43% | `java.lang.Enum.ordinal() line: 128` |
| 35 | 0.41% | `java.lang.ThreadLocal.get() line: 172` |
| 36 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 38 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 40 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.37% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 42 | 0.36% | `java.util.ArrayDeque.size() line: 648` |
| 43 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 44 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 46 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.31% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 48 | 0.30% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 49 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 50 | 0.30% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 51 | 0.29% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 53 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 54 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 55 | 0.27% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 56 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 57 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.26% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 59 | 0.26% | `io.netty.channel.AbstractChannelHandlerContext.executor() line: 132` |
| 60 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.82% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.80% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 10 | 0.87% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 12 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.69% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 14 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 15 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 18 | 0.62% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 19 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 20 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 21 | 0.59% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 22 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 23 | 0.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 24 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 29 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |

GC pauses: 0, total 0 ms


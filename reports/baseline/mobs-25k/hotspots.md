# JVM hotspots (JFR ExecutionSample)

Samples: 43776

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.60% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.17% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.89% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 13 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.76% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 17 | 0.73% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 18 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 29 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 30 | 0.48% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 31 | 0.47% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 32 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.46% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 527` |
| 34 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 35 | 0.45% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.45% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 39 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 40 | 0.43% | `java.util.ArrayDeque.size() line: 648` |
| 41 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.39% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 47 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.34% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 49 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 51 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 54 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 55 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 56 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 57 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 58 | 0.30% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.61% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.23% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.22% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 10 | 0.92% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 11 | 0.79% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 15 | 0.73% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 16 | 0.72% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 17 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 0.70% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 0.66% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 21 | 0.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 22 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 25 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 26 | 0.61% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 27 | 0.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 28 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 30 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms


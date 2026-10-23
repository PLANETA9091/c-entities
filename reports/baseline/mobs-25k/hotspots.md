# JVM hotspots (JFR ExecutionSample)

Samples: 52847

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.56% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.11% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.88% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 10 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 14 | 0.67% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 15 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.64% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 17 | 0.60% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 18 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 21 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 24 | 0.52% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 25 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.50% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.49% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 28 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.47% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 30 | 0.46% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 32 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 35 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 38 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 39 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 40 | 0.38% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 41 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.37% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 43 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 47 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 48 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 49 | 0.33% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 50 | 0.32% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.31% | `java.util.ArrayDeque.size() line: 648` |
| 52 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 53 | 0.29% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 54 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.29% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 56 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 57 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 58 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 59 | 0.28% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 60 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.57% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 3.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 0.90% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 9 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 11 | 0.76% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 14 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 15 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 16 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.60% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 24 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 25 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 26 | 0.57% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 27 | 0.53% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 28 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 29 | 0.52% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 30 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms


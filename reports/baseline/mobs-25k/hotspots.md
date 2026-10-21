# JVM hotspots (JFR ExecutionSample)

Samples: 64506

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.43% | `java.lang.ThreadLocal.get() line: 172` |
| 4 | 2.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.74% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 17 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 19 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 20 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.48% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 22 | 0.48% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 24 | 0.47% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 25 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 26 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 27 | 0.44% | `java.lang.Enum.ordinal() line: 128` |
| 28 | 0.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 30 | 0.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 33 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 35 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 36 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 39 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 40 | 0.33% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 41 | 0.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 42 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 43 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 44 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 45 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 46 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 47 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 51 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 52 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 53 | 0.26% | `java.util.ArrayDeque.size() line: 648` |
| 54 | 0.26% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 55 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 56 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 57 | 0.24% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 58 | 0.24% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 59 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.90% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.11% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 6 | 1.75% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 0.68% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 13 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 15 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 16 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 17 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 0.49% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 19 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.48% | `net.minecraft.world.entity.monster.Zombie.aiStep() line: 243` |
| 21 | 0.47% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 22 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.47% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.46% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 828` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 26 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |
| 27 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |

GC pauses: 0, total 0 ms


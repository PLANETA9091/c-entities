# JVM hotspots (JFR ExecutionSample)

Samples: 65745

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 2.82% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.52% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 0.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 0.60% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 17 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.51% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 20 | 0.50% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 21 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 22 | 0.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.44% | `java.lang.ThreadLocal.get() line: 172` |
| 25 | 0.43% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 27 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 29 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 30 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 31 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 34 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 35 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 36 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 37 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 38 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 39 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 40 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 41 | 0.33% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 42 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 43 | 0.30% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 44 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 45 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 46 | 0.28% | `java.util.ArrayDeque.size() line: 648` |
| 47 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 48 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 49 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 50 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 51 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 52 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.25% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 55 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 56 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 57 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 59 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 60 | 0.23% | `net.minecraft.util.Mth.floor(double) line: 66` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.83% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.52% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.70% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 13 | 0.64% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 14 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 15 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 16 | 0.52% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 17 | 0.51% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 19 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 20 | 0.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 21 | 0.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 22 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 27 | 0.43% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 28 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 30 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |

GC pauses: 0, total 0 ms


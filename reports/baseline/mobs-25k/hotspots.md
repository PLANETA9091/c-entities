# JVM hotspots (JFR ExecutionSample)

Samples: 63307

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 2.70% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.42% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 0.61% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 15 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 17 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.52% | `java.lang.ThreadLocal.get() line: 172` |
| 20 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 22 | 0.49% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 23 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 24 | 0.48% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 25 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 27 | 0.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.40% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 30 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 31 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 32 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 33 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 34 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 35 | 0.36% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 37 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 39 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 40 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 41 | 0.33% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 605` |
| 42 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 43 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 44 | 0.31% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 45 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 46 | 0.29% | `java.util.ArrayDeque.size() line: 648` |
| 47 | 0.28% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 48 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 49 | 0.26% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 50 | 0.26% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.25% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 53 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 54 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 55 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ThreadedTicketLevelPropagator.hasPendingUpdates() line: 115` |
| 59 | 0.22% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 60 | 0.21% | `java.util.EnumMap.isValidKey(Object) line: 319` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.69% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.43% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 12 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 13 | 0.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 15 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 16 | 0.52% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 17 | 0.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.50% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 19 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 20 | 0.49% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 21 | 0.48% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 828` |
| 22 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.46% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.46% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 26 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 30 | 0.42% | `java.lang.ThreadLocal.get(Thread) line: 187` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 64066

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.61% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 0.78% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 12 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.64% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 14 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 15 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.54% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 17 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.50% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 20 | 0.50% | `io.netty.util.internal.shaded.org.jctools.util.UnsafeRefArrayAccess.soRefElement(Object[], long, Object) line: 63` |
| 21 | 0.50% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 24 | 0.47% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 25 | 0.46% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.43% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 28 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 31 | 0.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 32 | 0.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 34 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 35 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 38 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 40 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 41 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.35% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 43 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 44 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 45 | 0.30% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 46 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 47 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 48 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 49 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 52 | 0.28% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 53 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 54 | 0.27% | `java.util.ArrayDeque.size() line: 648` |
| 55 | 0.27% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 56 | 0.27% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 57 | 0.26% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 58 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 499` |
| 59 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 60 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.62% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.19% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 7 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 0.79% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 9 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 13 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 14 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 16 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 17 | 0.50% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.50% | `io.netty.util.internal.shaded.org.jctools.queues.BaseMpscLinkedArrayQueue.offer(Object) line: 280` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 20 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 21 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 22 | 0.49% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 23 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 24 | 0.47% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.47% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 26 | 0.44% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 27 | 0.44% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 28 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |
| 29 | 0.43% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |

GC pauses: 0, total 0 ms


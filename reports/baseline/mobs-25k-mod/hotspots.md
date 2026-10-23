# JVM hotspots (JFR ExecutionSample)

Samples: 30247

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.89% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.52% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 4 | 2.41% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 2.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 6 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 1.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.00% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.91% | `java.util.EnumMap.get(Object) line: 247` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 18 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1217` |
| 23 | 0.63% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 24 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.62% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 26 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 27 | 0.60% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 30 | 0.57% | `java.util.ArrayDeque.size() line: 648` |
| 31 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 32 | 0.55% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.55% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 34 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 35 | 0.54% | `java.lang.ThreadLocal.get() line: 172` |
| 36 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 39 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 40 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 618` |
| 41 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 42 | 0.49% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 44 | 0.47% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 46 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 48 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 50 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 51 | 0.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 52 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 54 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 55 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 56 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 57 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.36% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 59 | 0.35% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 3 | 3.88% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1215` |
| 6 | 2.41% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 1.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 14 | 0.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 17 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 18 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 27 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.63% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 30 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


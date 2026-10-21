# JVM hotspots (JFR ExecutionSample)

Samples: 20906

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.31% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 3 | 2.04% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.22% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 15 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.77% | `java.lang.Enum.ordinal() line: 128` |
| 17 | 0.76% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 0.75% | `java.lang.ThreadLocal.get() line: 172` |
| 19 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 20 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 21 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 25 | 0.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.51% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 27 | 0.50% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 28 | 0.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 29 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 30 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 31 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 32 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 33 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 34 | 0.43% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 36 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 37 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 38 | 0.39% | `java.util.ArrayDeque.size() line: 648` |
| 39 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 41 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 42 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 43 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 46 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 48 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 49 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 50 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 51 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 54 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 55 | 0.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 56 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 57 | 0.27% | `net.minecraft.world.level.pathfinder.PathTypeCache.get(int, long) line: 23` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 59 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 60 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.33% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 2.04% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.92% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 12 | 0.81% | `java.util.EnumMap.get(Object) line: 247` |
| 13 | 0.76% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 14 | 0.76% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 16 | 0.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 19 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 25 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 26 | 0.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 27 | 0.54% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.53% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 30 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 764` |

GC pauses: 0, total 0 ms


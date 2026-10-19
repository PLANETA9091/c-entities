# JVM hotspots (JFR ExecutionSample)

Samples: 16296

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.74% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.20% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 6 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 7 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.95% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 0.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.92% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 0.86% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 18 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.82% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 605` |
| 20 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 22 | 0.65% | `java.lang.Enum.ordinal() line: 128` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.60% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 30 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 31 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.47% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 34 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 35 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 38 | 0.43% | `java.util.ArrayDeque.size() line: 648` |
| 39 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 40 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 41 | 0.38% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 42 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 43 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 45 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 46 | 0.32% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 47 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 49 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 51 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 54 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 55 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.26% | `net.minecraft.world.entity.Mob.isNoAi() line: 1471` |
| 57 | 0.26% | `net.minecraft.world.phys.Vec3.get(Direction$Axis) line: 270` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 59 | 0.25% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 60 | 0.25% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.04% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 1.74% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.21% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 7 | 1.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 9 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.94% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.87% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 14 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 15 | 0.82% | `net.minecraft.world.item.ItemStack.setDamageValue(int) line: 601` |
| 16 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 18 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 19 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 20 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 21 | 0.65% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 22 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 23 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 24 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.61% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 27 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 28 | 0.59% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 29 | 0.58% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |

GC pauses: 0, total 0 ms


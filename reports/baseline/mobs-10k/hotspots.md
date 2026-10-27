# JVM hotspots (JFR ExecutionSample)

Samples: 13717

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.11% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 1.89% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.25% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 9 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 12 | 0.95% | `java.util.EnumMap.get(Object) line: 247` |
| 13 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.82% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 17 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.82% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 19 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 22 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.59% | `java.lang.ThreadLocal.get() line: 172` |
| 24 | 0.58% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 25 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 26 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 28 | 0.55% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 29 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 30 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 31 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 32 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 33 | 0.47% | `java.lang.Enum.ordinal() line: 128` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 37 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1315` |
| 40 | 0.39% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.37% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 43 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.36% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 45 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 501` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 47 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 48 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 49 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 50 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 51 | 0.32% | `java.util.ArrayDeque.size() line: 648` |
| 52 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 54 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 56 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 58 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 60 | 0.27% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.11% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 1.89% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.75% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.16% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 11 | 1.12% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 12 | 0.91% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.82% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 14 | 0.82% | `net.minecraft.world.entity.monster.Zombie.aiStep() line: 243` |
| 15 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 16 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 17 | 0.76% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 21 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 23 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 27 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 28 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.58% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |

GC pauses: 0, total 0 ms


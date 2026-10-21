# JVM hotspots (JFR ExecutionSample)

Samples: 13768

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 1.84% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 8 | 1.13% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.00% | `java.lang.ThreadLocal.get() line: 172` |
| 12 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 17 | 0.79% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 20 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.55% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 31 | 0.52% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 33 | 0.51% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 34 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 35 | 0.47% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 36 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 37 | 0.45% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.44% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 42 | 0.40% | `java.util.ArrayDeque.size() line: 648` |
| 43 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.37% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 46 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 47 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.34% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.33% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.32% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 54 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 56 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 57 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 500` |
| 58 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 501` |
| 59 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 60 | 0.31% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 1.84% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 1.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 9 | 1.13% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 10 | 1.01% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 0.94% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 13 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 0.86% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 15 | 0.86% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 17 | 0.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.78% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 23 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 25 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 30 | 0.57% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |

GC pauses: 0, total 0 ms


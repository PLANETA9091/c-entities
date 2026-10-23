# JVM hotspots (JFR ExecutionSample)

Samples: 35921

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.68% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.07% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.05% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 0.98% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 0.84% | `java.util.EnumMap.get(Object) line: 247` |
| 12 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 0.78% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.77% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 18 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.55% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 25 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.50% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 28 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 29 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 30 | 0.48% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 31 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 33 | 0.46% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 34 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 35 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.45% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 37 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 38 | 0.45% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 39 | 0.44% | `java.lang.ThreadLocal.get() line: 172` |
| 40 | 0.44% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 41 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 43 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 44 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 47 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 48 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 49 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 51 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 53 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 54 | 0.38% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 55 | 0.38% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 56 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 58 | 0.34% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 59 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.69% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.08% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.98% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 11 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 12 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 0.78% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 16 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 17 | 0.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.59% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 24 | 0.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 26 | 0.55% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.53% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 29 | 0.51% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 30 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |

GC pauses: 0, total 0 ms


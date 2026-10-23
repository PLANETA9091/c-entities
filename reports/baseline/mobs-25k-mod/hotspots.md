# JVM hotspots (JFR ExecutionSample)

Samples: 33259

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.48% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.36% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.34% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 10 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 12 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.90% | `java.util.EnumMap.get(Object) line: 247` |
| 14 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 17 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 20 | 0.70% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 23 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 25 | 0.57% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.56% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.53% | `java.lang.Enum.ordinal() line: 128` |
| 29 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 30 | 0.51% | `java.lang.ThreadLocal.get() line: 172` |
| 31 | 0.51% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 32 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 33 | 0.49% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 34 | 0.49% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 35 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 37 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 38 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 39 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 40 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 41 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 43 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 45 | 0.40% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 46 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 47 | 0.38% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 48 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.38% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1200` |
| 51 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 52 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 53 | 0.36% | `java.util.ArrayDeque.size() line: 648` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.34% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 56 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 57 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 58 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 417` |
| 59 | 0.30% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 60 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.51% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.37% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.03% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 10 | 0.98% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 13 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 931` |
| 17 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 18 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 20 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 21 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 23 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.66% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.58% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 30 | 0.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |

GC pauses: 0, total 0 ms


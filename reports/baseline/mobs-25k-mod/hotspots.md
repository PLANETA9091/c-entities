# JVM hotspots (JFR ExecutionSample)

Samples: 31770

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.44% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 1.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1017` |
| 4 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 8 | 1.06% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 0.98% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 0.93% | `java.util.EnumMap.get(Object) line: 247` |
| 12 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 15 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.81% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 17 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.72% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 19 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 20 | 0.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 21 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 0.60% | `java.lang.ThreadLocal.get() line: 172` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 29 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 31 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 32 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 33 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 34 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 35 | 0.54% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 37 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 38 | 0.53% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 39 | 0.52% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 41 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 42 | 0.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 43 | 0.50% | `java.util.ArrayDeque.size() line: 648` |
| 44 | 0.50% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 754` |
| 45 | 0.49% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 46 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 47 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 48 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 49 | 0.47% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 50 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 51 | 0.46% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 52 | 0.44% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 53 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 54 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 55 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 56 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 57 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 58 | 0.38% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 59 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 60 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.46% | `java.util.HashMap.containsKey(Object) line: 602` |
| 3 | 2.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 544` |
| 5 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.09% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 0.97% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 9 | 0.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 10 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 11 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 14 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 15 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 16 | 0.81% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 17 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 19 | 0.74% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 20 | 0.73% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 22 | 0.72% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 24 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |

GC pauses: 0, total 0 ms


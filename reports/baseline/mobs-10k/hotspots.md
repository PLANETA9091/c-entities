# JVM hotspots (JFR ExecutionSample)

Samples: 13891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.38% | `java.lang.ThreadLocal.get() line: 172` |
| 3 | 1.96% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.33% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 7 | 1.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 11 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.89% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 0.87% | `java.util.EnumMap.get(Object) line: 247` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.68% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 19 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 23 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 24 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 25 | 0.53% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 26 | 0.53% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 27 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 30 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 32 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 34 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 35 | 0.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 36 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 37 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 38 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.42% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 40 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 41 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 42 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 43 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 44 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.37% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.36% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 47 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 50 | 0.33% | `java.util.ArrayDeque.size() line: 648` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 52 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 53 | 0.31% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 54 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 55 | 0.30% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 57 | 0.29% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 58 | 0.29% | `io.netty.channel.AbstractChannelHandlerContext.executor() line: 132` |
| 59 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.17% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 4 | 1.96% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.43% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.36% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 9 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.94% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 0.91% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 12 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 15 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 16 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 18 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 23 | 0.63% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 26 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 27 | 0.58% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |
| 28 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |

GC pauses: 0, total 0 ms


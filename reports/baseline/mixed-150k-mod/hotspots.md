# JVM hotspots (JFR ExecutionSample)

Samples: 47532

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.06% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 2.40% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 2.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.64% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 9 | 1.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.49% | `java.lang.ThreadLocal.get() line: 172` |
| 12 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 15 | 1.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 16 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 17 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 19 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 20 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 22 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 23 | 0.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 25 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 26 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 28 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 30 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 31 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 32 | 0.67% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 33 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 35 | 0.60% | `java.util.ArrayDeque.size() line: 648` |
| 36 | 0.57% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 38 | 0.53% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 39 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 40 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 41 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 42 | 0.50% | `java.lang.Enum.ordinal() line: 128` |
| 43 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 45 | 0.45% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 46 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 48 | 0.40% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 49 | 0.40% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 51 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 54 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 136` |
| 56 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 57 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 59 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 60 | 0.30% | `net.minecraft.core.Vec3i.getZ() line: 71` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.09% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 4 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 2.40% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 2.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 10 | 1.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 11 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 12 | 1.50% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 13 | 1.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 15 | 1.31% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 16 | 1.25% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 1.18% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 20 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 21 | 0.99% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 23 | 0.91% | `java.util.HashMap.containsKey(Object) line: 602` |
| 24 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 25 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 26 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 27 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 29 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |

GC pauses: 0, total 0 ms


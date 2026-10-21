# JVM hotspots (JFR ExecutionSample)

Samples: 99338

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.92% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 20 | 0.75% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 21 | 0.73% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 22 | 0.71% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 23 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 24 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 25 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 26 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 28 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 29 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 30 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.49% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 34 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 35 | 0.45% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 36 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 38 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 40 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 42 | 0.37% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 43 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 45 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 46 | 0.32% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.31% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 49 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 51 | 0.29% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 52 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 53 | 0.28% | `java.lang.ThreadLocal.get() line: 172` |
| 54 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 55 | 0.26% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 56 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 58 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 60 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.92% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.26% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 13 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 14 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 0.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 17 | 0.76% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.73% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 0.72% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 22 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 23 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.64% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 26 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 27 | 0.59% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 28 | 0.51% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 29 | 0.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 30 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


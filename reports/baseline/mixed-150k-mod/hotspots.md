# JVM hotspots (JFR ExecutionSample)

Samples: 87412

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.68% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 14 | 0.94% | `java.lang.ThreadLocal.get() line: 172` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 23 | 0.80% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 26 | 0.76% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, AABB) line: 235` |
| 31 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.50% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 33 | 0.49% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 35 | 0.43% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 36 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 37 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 39 | 0.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 40 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 45 | 0.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 46 | 0.33% | `java.util.ArrayDeque.size() line: 648` |
| 47 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 48 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 49 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 50 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 55 | 0.26% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 58 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.25% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 60 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.69% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 18 | 0.85% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.80% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.79% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 23 | 0.77% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.resumeIndexing() line: 424` |
| 25 | 0.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 26 | 0.63% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 28 | 0.50% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 29 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |

GC pauses: 0, total 0 ms


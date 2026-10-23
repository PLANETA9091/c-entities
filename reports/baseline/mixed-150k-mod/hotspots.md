# JVM hotspots (JFR ExecutionSample)

Samples: 56779

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 3 | 2.78% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 7 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.77% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.63% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 1.58% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 16 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.18% | `java.lang.ThreadLocal.get() line: 172` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 21 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 1.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 25 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 26 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 27 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.71% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 32 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 33 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 34 | 0.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 35 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 36 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.59% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 38 | 0.58% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 39 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 40 | 0.54% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 44 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.48% | `java.util.ArrayDeque.size() line: 648` |
| 46 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 47 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 49 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 51 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 52 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 55 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 56 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 57 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 59 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 4 | 2.80% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 6 | 2.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 2.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 9 | 2.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 1.77% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 1.66% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.47% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 17 | 1.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 18 | 1.20% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 1.03% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 22 | 1.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.91% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 26 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3320` |
| 28 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 29 | 0.74% | `net.minecraft.world.entity.EntitySelector.lambda$pushable$9(boolean, Entity, Team, Team$CollisionRule, Entity) line: 58` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |

GC pauses: 0, total 0 ms


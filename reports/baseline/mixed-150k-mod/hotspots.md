# JVM hotspots (JFR ExecutionSample)

Samples: 59729

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 3.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.79% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 2.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 9 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 10 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 1.38% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.13% | `java.lang.ThreadLocal.get() line: 172` |
| 16 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 17 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 18 | 1.00% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 19 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 22 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 23 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.76% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 25 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.67% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 32 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 33 | 0.54% | `net.minecraft.world.entity.Entity.getRootVehicle() line: 4556` |
| 34 | 0.53% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 35 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 39 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.47% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.46% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 44 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.43% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 46 | 0.39% | `java.util.ArrayDeque.size() line: 648` |
| 47 | 0.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 49 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 51 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 54 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 57 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 59 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 60 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 3.30% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 6 | 2.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.80% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 2.77% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 9 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 1.38% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 1.38% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 14 | 1.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 16 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 17 | 1.11% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 1.05% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 1.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 20 | 1.01% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 22 | 0.87% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1432` |
| 26 | 0.70% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 28 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 3842` |

GC pauses: 0, total 0 ms


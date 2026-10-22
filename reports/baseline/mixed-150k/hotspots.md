# JVM hotspots (JFR ExecutionSample)

Samples: 113373

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.70% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.94% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.73% | `java.lang.ThreadLocal.get() line: 172` |
| 15 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.69% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 18 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 23 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 25 | 0.47% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 29 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.41% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 31 | 0.41% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 32 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 35 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 37 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 40 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 41 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 43 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 44 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 45 | 0.27% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.25% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 48 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 50 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 51 | 0.22% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 52 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 53 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 54 | 0.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 55 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 56 | 0.20% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.19% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 60 | 0.19% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 3.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 1.71% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 8 | 1.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 11 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.96% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 0.67% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.61% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 21 | 0.59% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 22 | 0.53% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.47% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 25 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 29 | 0.40% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |

GC pauses: 0, total 0 ms


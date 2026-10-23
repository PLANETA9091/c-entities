# JVM hotspots (JFR ExecutionSample)

Samples: 104003

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.81% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.11% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 11 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.89% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 16 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 0.75% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 21 | 0.66% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.62% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 26 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 27 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 28 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 29 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 32 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 33 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 34 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 35 | 0.43% | `java.lang.ThreadLocal.get() line: 172` |
| 36 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 37 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 40 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.35% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 46 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 48 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 49 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 50 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 51 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 52 | 0.31% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 53 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 54 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.29% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 56 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 58 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 136` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.81% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.42% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.12% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 12 | 0.86% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 13 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 16 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 0.71% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.70% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 25 | 0.62% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 28 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 29 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.48% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms


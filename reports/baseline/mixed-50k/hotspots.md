# JVM hotspots (JFR ExecutionSample)

Samples: 13595

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.06% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 1.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.57% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.21% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 15 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 19 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 26 | 0.72% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 28 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 31 | 0.61% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 32 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 36 | 0.51% | `java.lang.Enum.ordinal() line: 128` |
| 37 | 0.50% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 38 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 39 | 0.50% | `java.util.EnumMap.get(Object) line: 247` |
| 40 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 42 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 45 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 47 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 50 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 52 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 54 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 55 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 56 | 0.32% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 60 | 0.31% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 4 | 2.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 6 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.57% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 1.18% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 15 | 1.06% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 16 | 1.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.90% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |

GC pauses: 0, total 0 ms


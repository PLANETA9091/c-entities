# JVM hotspots (JFR ExecutionSample)

Samples: 99857

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.89% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.22% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 1.16% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 16 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 20 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 24 | 0.54% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 25 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 26 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 30 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 31 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 32 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 33 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 34 | 0.40% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 35 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.40% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.39% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 38 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 40 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 41 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 42 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 43 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 45 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 46 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 47 | 0.28% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 49 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 52 | 0.25% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.24% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 55 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 56 | 0.23% | `java.lang.ThreadLocal.get() line: 172` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 58 | 0.22% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 59 | 0.22% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 60 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 12.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.89% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 8 | 1.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.22% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 1.17% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.99% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 14 | 0.86% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 18 | 0.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.50% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 29 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 30 | 0.41% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |

GC pauses: 0, total 0 ms


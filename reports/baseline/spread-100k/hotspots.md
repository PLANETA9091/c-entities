# JVM hotspots (JFR ExecutionSample)

Samples: 70462

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.90% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.10% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 17 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 19 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 21 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.79% | `java.lang.Enum.ordinal() line: 128` |
| 25 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 29 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.64% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 31 | 0.63% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 32 | 0.61% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 34 | 0.59% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 35 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 36 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 37 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 42 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 43 | 0.45% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 47 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 49 | 0.38% | `java.lang.ThreadLocal.get() line: 172` |
| 50 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 51 | 0.36% | `java.util.EnumMap.get(Object) line: 247` |
| 52 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 53 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 54 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 55 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 56 | 0.32% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.31% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 60 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.08% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 6 | 1.91% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.51% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 16 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 18 | 0.87% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 20 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.72% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.63% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.60% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 30 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |

GC pauses: 0, total 0 ms


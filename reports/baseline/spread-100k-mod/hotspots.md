# JVM hotspots (JFR ExecutionSample)

Samples: 64077

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 3 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 11 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry.getValueVolatile() line: 1797` |
| 12 | 1.01% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 13 | 0.99% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 17 | 0.95% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 18 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 22 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 23 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 24 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 25 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 26 | 0.83% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 27 | 0.81% | `java.lang.Enum.ordinal() line: 128` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 31 | 0.65% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 32 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 33 | 0.64% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.62% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 35 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.53% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 40 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 42 | 0.44% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 43 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 44 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 46 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3597` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 49 | 0.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 50 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 51 | 0.39% | `java.lang.ThreadLocal.get() line: 172` |
| 52 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 53 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 55 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 56 | 0.36% | `java.util.EnumMap.get(Object) line: 247` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 60 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.03% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 7 | 1.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 1.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 13 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 16 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 225` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.90% | `java.util.HashMap.containsKey(Object) line: 602` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 21 | 0.86% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.84% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 26 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.65% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 29 | 0.63% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 59061

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 1.96% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.36% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.21% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 21 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 24 | 0.80% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 25 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 26 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 27 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 30 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 31 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.62% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 34 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 35 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 37 | 0.56% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 38 | 0.54% | `java.util.EnumMap.get(Object) line: 247` |
| 39 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.53% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 41 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 43 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 45 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 46 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 47 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 48 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 49 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 50 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 52 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 53 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 54 | 0.36% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 55 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 56 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 57 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 58 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 59 | 0.35% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 60 | 0.34% | `java.util.ArrayDeque.size() line: 648` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.04% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 1.97% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.61% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 14 | 1.36% | `java.util.HashMap.containsKey(Object) line: 602` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 17 | 1.21% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 18 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 25 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 27 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 63175

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 3 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 1.40% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 14 | 1.08% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.81% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 21 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 22 | 0.73% | `java.util.HashMap.getNode(Object) line: 585` |
| 23 | 0.70% | `java.lang.Enum.ordinal() line: 128` |
| 24 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 25 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 32 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 33 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 34 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 36 | 0.48% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 37 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 38 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 39 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 40 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 41 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.44% | `java.lang.ThreadLocal.get() line: 172` |
| 43 | 0.44% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 45 | 0.44% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 46 | 0.41% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(int, int) line: 32` |
| 47 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1315` |
| 48 | 0.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 49 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 50 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 52 | 0.34% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 55 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 56 | 0.31% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 58 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 60 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 3 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.42% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 16 | 1.09% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 17 | 1.07% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.85% | `java.util.EnumMap.get(Object) line: 247` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.77% | `java.util.HashMap.containsKey(Object) line: 602` |
| 23 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 29 | 0.58% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |

GC pauses: 0, total 0 ms


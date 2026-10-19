# JVM hotspots (JFR ExecutionSample)

Samples: 66268

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 0.87% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 0.83% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 0.77% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 18 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 19 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 25 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 26 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 27 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.46% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 30 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 31 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 32 | 0.41% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 34 | 0.38% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 35 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 37 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 38 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 41 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 43 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 45 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 47 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 48 | 0.24% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 49 | 0.23% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 50 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 51 | 0.23% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 52 | 0.22% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 54 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 55 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 56 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.20% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.99% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.88% | `java.util.HashMap.containsKey(Object) line: 602` |
| 13 | 0.77% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 14 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.68% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 16 | 0.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 20 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 21 | 0.53% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 23 | 0.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 24 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 25 | 0.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 26 | 0.44% | `java.util.EnumMap.get(Object) line: 246` |
| 27 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 28 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 29 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms


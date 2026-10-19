# JVM hotspots (JFR ExecutionSample)

Samples: 74470

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 42.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 0.89% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 7 | 0.89% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.85% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 0.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.63% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 17 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 18 | 0.57% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 19 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 0.56% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.50% | `java.lang.ThreadLocal.get() line: 172` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.46% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 27 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 28 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 31 | 0.38% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 32 | 0.38% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 34 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 36 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 37 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 39 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 40 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 41 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.27% | `java.util.HashMap.getNode(Object) line: 585` |
| 44 | 0.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 45 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 46 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 47 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 48 | 0.23% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.22% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 50 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 51 | 0.21% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 52 | 0.21% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 54 | 0.20% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 55 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 57 | 0.19% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 58 | 0.18% | `java.util.EnumMap.get(Object) line: 247` |
| 59 | 0.18% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 60 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 42.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.89% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 0.85% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 13 | 0.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 16 | 0.58% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 18 | 0.53% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 23 | 0.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 24 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.39% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 28 | 0.37% | `java.util.EnumMap.get(Object) line: 246` |
| 29 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms


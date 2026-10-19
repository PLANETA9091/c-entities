# JVM hotspots (JFR ExecutionSample)

Samples: 15894

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 6 | 1.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.26% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 9 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.13% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 15 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.90% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 18 | 0.89% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 20 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 21 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 23 | 0.72% | `java.lang.ThreadLocal.get() line: 172` |
| 24 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 25 | 0.67% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.59% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 28 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 29 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 30 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 32 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 33 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 34 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 36 | 0.48% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 38 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 39 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 40 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 41 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 42 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.34% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 46 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 48 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 49 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 51 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 52 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.31% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 54 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 55 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 56 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 58 | 0.28% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 59 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.28% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 121` |
| 13 | 1.13% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 0.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 17 | 0.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 0.77% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 21 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 22 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 24 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 25 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 26 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 27 | 0.59% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 29 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |

GC pauses: 0, total 0 ms


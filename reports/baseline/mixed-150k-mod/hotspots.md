# JVM hotspots (JFR ExecutionSample)

Samples: 98984

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.80% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.07% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.00% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |
| 16 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 17 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 22 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 24 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 27 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.54% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.48% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 35 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 36 | 0.47% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 37 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 40 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 42 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 43 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 44 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 46 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 47 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 49 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 50 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 51 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 52 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 53 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 54 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 56 | 0.25% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 57 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 60 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.81% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.29% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.09% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 1.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 13 | 1.00% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 14 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 0.96% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.80% | `java.util.EnumMap.get(Object) line: 247` |
| 17 | 0.76% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.70% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 0.70% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 21 | 0.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 23 | 0.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 27 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 28 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 29 | 0.50% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 112277

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.77% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.24% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 0.96% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 12 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.77% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 0.74% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 17 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.64% | `java.lang.ThreadLocal.get() line: 172` |
| 21 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 23 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 24 | 0.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 25 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 26 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.53% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 28 | 0.49% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.48% | `java.lang.Enum.ordinal() line: 128` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.46% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 32 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 33 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 34 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 35 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 36 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 37 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.37% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 40 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 41 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 42 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 44 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 45 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 46 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 47 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 48 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 50 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 53 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 55 | 0.22% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 56 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 449` |
| 57 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 58 | 0.21% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 59 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 60 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 1.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.78% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 8 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 11 | 0.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 0.98% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.77% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 15 | 0.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.66% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.57% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.56% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.50% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 26 | 0.47% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 29 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.40% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |

GC pauses: 0, total 0 ms


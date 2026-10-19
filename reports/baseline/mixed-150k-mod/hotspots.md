# JVM hotspots (JFR ExecutionSample)

Samples: 70669

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 0.95% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 0.83% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 0.78% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 18 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.50% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 23 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 25 | 0.48% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 26 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 29 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 31 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 32 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 33 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.39% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 35 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.38% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.37% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 40 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 41 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 44 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 46 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.26% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 49 | 0.25% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 50 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 53 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 54 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 56 | 0.22% | `java.util.ArrayDeque.size() line: 648` |
| 57 | 0.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 58 | 0.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 59 | 0.19% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.98% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 0.96% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 0.93% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.77% | `java.util.HashMap.containsKey(Object) line: 602` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 16 | 0.67% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.63% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.62% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 20 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.54% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 22 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 24 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 25 | 0.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 26 | 0.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 27 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 28 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 29 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.39% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 35979

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 3 | 2.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.32% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.06% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 14 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.00% | `java.util.HashMap.getNode(Object) line: 585` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 18 | 0.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 20 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 23 | 0.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 24 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 27 | 0.68% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 30 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 32 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 33 | 0.58% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 34 | 0.58% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 35 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 36 | 0.57% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 37 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 45 | 0.43% | `java.lang.ThreadLocal.get() line: 172` |
| 46 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 48 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 49 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 50 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 51 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 52 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 53 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 54 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 55 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 56 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 57 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.34% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 59 | 0.34% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.34% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 1.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.59% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.38% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 1.32% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 15 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.00% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 19 | 0.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 21 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 22 | 0.69% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 23 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 24 | 0.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 26 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 27 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms


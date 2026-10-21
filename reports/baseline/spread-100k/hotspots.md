# JVM hotspots (JFR ExecutionSample)

Samples: 69389

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.73% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.01% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 18 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 19 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 23 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 25 | 0.80% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 26 | 0.79% | `java.util.HashMap.getNode(Object) line: 585` |
| 27 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 28 | 0.75% | `java.lang.Enum.ordinal() line: 128` |
| 29 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 34 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 35 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 37 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 40 | 0.47% | `java.util.EnumMap.get(Object) line: 247` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 42 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 43 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.43% | `io.netty.util.internal.shaded.org.jctools.util.UnsafeRefArrayAccess.soRefElement(Object[], long, Object) line: 63` |
| 45 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 46 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 47 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 48 | 0.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 50 | 0.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 51 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 55 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 56 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 57 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 58 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 59 | 0.32% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.76% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 8 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 13 | 1.43% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.06% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 19 | 1.01% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.86% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.81% | `java.util.HashMap.containsKey(Object) line: 602` |
| 24 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.72% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 28 | 0.58% | `java.util.EnumMap.get(Object) line: 247` |
| 29 | 0.58% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

GC pauses: 0, total 0 ms


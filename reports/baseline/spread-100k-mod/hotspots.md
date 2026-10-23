# JVM hotspots (JFR ExecutionSample)

Samples: 52888

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 1.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.90% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 1.55% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.51% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 11 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 14 | 1.06% | `java.util.HashMap.getNode(Object) line: 585` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 16 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 17 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 18 | 0.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 21 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 24 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 25 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 26 | 0.72% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 29 | 0.69% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 30 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 31 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 32 | 0.63% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 33 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 35 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 38 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 40 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 41 | 0.49% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 42 | 0.48% | `java.lang.ThreadLocal.get() line: 172` |
| 43 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 46 | 0.42% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 49 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 50 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 51 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 53 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 54 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 55 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 56 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 57 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 60 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 4 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 2.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.90% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 10 | 1.52% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 11 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.30% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 15 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.07% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 20 | 0.88% | `java.util.EnumMap.get(Object) line: 247` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 25 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 29 | 0.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |

GC pauses: 0, total 0 ms


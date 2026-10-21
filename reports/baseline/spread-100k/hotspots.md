# JVM hotspots (JFR ExecutionSample)

Samples: 79871

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.23% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.16% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 11 | 1.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 15 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 16 | 0.94% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 21 | 0.86% | `java.lang.Enum.ordinal() line: 128` |
| 22 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.81% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 27 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 29 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.64% | `java.util.HashMap.getNode(Object) line: 585` |
| 31 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.62% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 34 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 35 | 0.49% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 37 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 38 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.45% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 40 | 0.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 41 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 44 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 45 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.37% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 47 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 49 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 50 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 53 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 54 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 55 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 56 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 57 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 59 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.46% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 5 | 1.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 1.50% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.23% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 15 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.95% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 19 | 0.94% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 20 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.68% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 26 | 0.65% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.65% | `java.util.HashMap.containsKey(Object) line: 602` |
| 28 | 0.63% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 111343

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 9.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 8 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 9 | 1.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 10 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.92% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 0.92% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 0.58% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.58% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 25 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 26 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 27 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 32 | 0.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 34 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 35 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 36 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 37 | 0.33% | `java.util.HashMap.getNode(Object) line: 585` |
| 38 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 39 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 40 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 42 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 44 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 45 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 46 | 0.24% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 48 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 49 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 51 | 0.22% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.22% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 54 | 0.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 55 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 56 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 57 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 59 | 0.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 60 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 13.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 7 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 9 | 1.27% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 10 | 0.96% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 0.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.68% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 18 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 19 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 23 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 24 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.40% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 28 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 29 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms


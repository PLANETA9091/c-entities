# JVM hotspots (JFR ExecutionSample)

Samples: 116189

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.72% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.31% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.16% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 0.95% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 14 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.72% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 19 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 20 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 21 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 23 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 26 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 27 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 28 | 0.49% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 29 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 31 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 33 | 0.45% | `java.lang.Enum.ordinal() line: 128` |
| 34 | 0.45% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 35 | 0.44% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 36 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 37 | 0.41% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 38 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 40 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 41 | 0.34% | `java.lang.ThreadLocal.get() line: 172` |
| 42 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.30% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 46 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 47 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 53 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 57 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 58 | 0.22% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 59 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 1.72% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 11 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.97% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 0.85% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 0.76% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.67% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 24 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.46% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.46% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.44% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 29 | 0.44% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 30 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |

GC pauses: 0, total 0 ms


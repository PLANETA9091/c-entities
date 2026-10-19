# JVM hotspots (JFR ExecutionSample)

Samples: 149340

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 42.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.23% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 0.76% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 12 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 0.62% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 18 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 0.53% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 24 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 25 | 0.47% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 26 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 29 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 31 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 33 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 34 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 35 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 36 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 37 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 38 | 0.32% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 39 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 40 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 41 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 42 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 44 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 45 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.25% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 49 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet$SetIterator.next() line: 522` |
| 52 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 54 | 0.23% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 55 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 56 | 0.22% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 57 | 0.21% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 59 | 0.20% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 60 | 0.20% | `java.util.EnumMap.isValidKey(Object) line: 319` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 42.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 1.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.23% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 0.91% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 0.76% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 13 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 0.65% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 20 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.48% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 23 | 0.46% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 26 | 0.41% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 28 | 0.33% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 828` |
| 29 | 0.33% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.33% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3320` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 64495

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.65% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 1.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 11 | 1.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 12 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 0.84% | `java.lang.ThreadLocal.get() line: 172` |
| 17 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 18 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 19 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 20 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 22 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 24 | 0.70% | `java.lang.Enum.ordinal() line: 128` |
| 25 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 27 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 32 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 33 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 34 | 0.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 994` |
| 36 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 38 | 0.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 39 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 45 | 0.41% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 47 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 49 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 50 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 52 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 55 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 56 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 58 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 60 | 0.31% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.65% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.59% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 7 | 1.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.34% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 10 | 1.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.31% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 12 | 1.27% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 13 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 16 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 17 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 18 | 0.87% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.85% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1462` |
| 22 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.69% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |

GC pauses: 0, total 0 ms


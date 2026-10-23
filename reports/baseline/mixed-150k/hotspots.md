# JVM hotspots (JFR ExecutionSample)

Samples: 62097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 2.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 8 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 18 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.87% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 23 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 25 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.74% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 30 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.55% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 34 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 40 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.39% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 46 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 50 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 52 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 53 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.29% | `java.util.ArrayDeque.size() line: 647` |
| 56 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.26% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 59 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 6.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.73% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 12 | 1.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.19% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 16 | 0.87% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 17 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 19 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 26 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.50% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |

GC pauses: 0, total 0 ms


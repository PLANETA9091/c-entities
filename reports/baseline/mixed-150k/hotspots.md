# JVM hotspots (JFR ExecutionSample)

Samples: 63877

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 9 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 14 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 16 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.99% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 21 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.73% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 26 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 28 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 30 | 0.58% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 31 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.57% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 33 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 35 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.46% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 37 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 42 | 0.42% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.37% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 49 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 50 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 52 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 55 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 57 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.commonTick() line: 881` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 12 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.05% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 17 | 0.80% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 19 | 0.75% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.73% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.67% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.67% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 23 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.58% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 26 | 0.57% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.47% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |

GC pauses: 0, total 0 ms


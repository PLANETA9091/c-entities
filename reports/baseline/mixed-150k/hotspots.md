# JVM hotspots (JFR ExecutionSample)

Samples: 60679

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 3.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 9 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.50% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 12 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.25% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 16 | 1.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 18 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 22 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 24 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.53% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 31 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 32 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.45% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 34 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 36 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.37% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 40 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 45 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 46 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 49 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1423` |
| 51 | 0.29% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 52 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 54 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 55 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.26% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.25% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 4.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 12 | 1.25% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 0.75% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 19 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 25 | 0.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 27 | 0.47% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 28 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

GC pauses: 0, total 0 ms


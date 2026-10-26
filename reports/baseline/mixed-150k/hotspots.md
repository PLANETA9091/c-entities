# JVM hotspots (JFR ExecutionSample)

Samples: 64355

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 2.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 11 | 1.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.70% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 36 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.39% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 39 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 41 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 43 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 46 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 52 | 0.29% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 53 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 54 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.26% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 60 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 4.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.35% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 0.70% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 19 | 0.62% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 22 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 26 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 27 | 0.42% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.40% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.39% | `net.minecraft.world.entity.LivingEntity.tick() line: 3481` |

GC pauses: 0, total 0 ms


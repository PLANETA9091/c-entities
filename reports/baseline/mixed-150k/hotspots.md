# JVM hotspots (JFR ExecutionSample)

Samples: 60597

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.01% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 17 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 21 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.75% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 31 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.49% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 33 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 36 | 0.44% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 40 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.40% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 44 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 46 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.34% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 50 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 52 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.28% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 56 | 0.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.24% | `java.util.ArrayDeque.size() line: 647` |
| 60 | 0.24% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 6.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 6.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 4.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 3.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.16% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 16 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.60% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.51% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 26 | 0.49% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.45% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms


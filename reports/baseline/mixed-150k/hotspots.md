# JVM hotspots (JFR ExecutionSample)

Samples: 58814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 1.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 1.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 17 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.72% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 31 | 0.51% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.48% | `java.lang.String.equals(Object) line: 1926` |
| 33 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 34 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 35 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.42% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 39 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.37% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 42 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 44 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 45 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 49 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 50 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 52 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 56 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.26% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.25% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 7.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 6.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 2.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.80% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 17 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.73% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 22 | 0.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.54% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 25 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.52% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 27 | 0.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 28 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 30 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


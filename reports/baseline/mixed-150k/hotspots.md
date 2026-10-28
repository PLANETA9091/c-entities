# JVM hotspots (JFR ExecutionSample)

Samples: 69867

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 16 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 21 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 23 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 34 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 36 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 39 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 44 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 45 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 46 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 48 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.26% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 53 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 55 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.23% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 60 | 0.21% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 5.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.36% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.22% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 19 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.59% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.50% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 26 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


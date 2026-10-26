# JVM hotspots (JFR ExecutionSample)

Samples: 59433

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 2.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.63% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 11 | 1.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 1.26% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 21 | 1.07% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 25 | 0.76% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.54% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 33 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.49% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.40% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 38 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 44 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 46 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.29% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 53 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 54 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 56 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 58 | 0.25% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1843` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.63% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 1.36% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.56% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.56% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 26 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.49% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 28 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |

GC pauses: 0, total 0 ms


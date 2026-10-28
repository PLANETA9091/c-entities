# JVM hotspots (JFR ExecutionSample)

Samples: 76534

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 3.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 3.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.19% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 14 | 1.09% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 17 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 19 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.57% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 32 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 33 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 34 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 36 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 37 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 39 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 40 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 42 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 47 | 0.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 50 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 51 | 0.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.24% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.24% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 57 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 59 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 5.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 3.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.30% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.19% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 1.10% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.50% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.42% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 27 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.35% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 29 | 0.34% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


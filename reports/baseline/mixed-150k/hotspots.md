# JVM hotspots (JFR ExecutionSample)

Samples: 69336

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 4.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 30 | 0.50% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 31 | 0.49% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.48% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 38 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 42 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 43 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 49 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 50 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.26% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.24% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 5.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.05% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.85% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 18 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.50% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 22 | 0.49% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 25 | 0.46% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.39% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 30 | 0.37% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms


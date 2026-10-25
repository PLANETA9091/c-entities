# JVM hotspots (JFR ExecutionSample)

Samples: 91016

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 4.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 3.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.69% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 10 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 0.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.83% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.47% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.45% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 33 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 35 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.32% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 38 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 39 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 43 | 0.28% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 44 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 47 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 51 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 53 | 0.22% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.20% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.20% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 60 | 0.18% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 10 | 1.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.98% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.51% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.46% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 24 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.35% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 29 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


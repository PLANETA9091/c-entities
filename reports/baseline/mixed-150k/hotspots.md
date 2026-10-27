# JVM hotspots (JFR ExecutionSample)

Samples: 94324

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.94% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.41% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 31 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 35 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 36 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.26% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.24% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 50 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.23% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 53 | 0.23% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.23% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 55 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 57 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 60 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 1.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.87% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 0.94% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.57% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.45% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 25 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.39% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.32% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms


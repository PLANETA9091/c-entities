# JVM hotspots (JFR ExecutionSample)

Samples: 72340

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 3.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.30% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.10% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 20 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.51% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 32 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 33 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.39% | `java.lang.String.equals(Object) line: 1926` |
| 36 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.38% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 38 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 40 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 42 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 46 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 49 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 52 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 54 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 56 | 0.25% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 57 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1465` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 5.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 11 | 1.30% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 14 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 16 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 20 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.51% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 27 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.40% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

GC pauses: 0, total 0 ms


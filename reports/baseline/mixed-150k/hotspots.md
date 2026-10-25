# JVM hotspots (JFR ExecutionSample)

Samples: 74607

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 2.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.06% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 17 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.55% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 28 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 30 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 32 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.41% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 37 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 40 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 41 | 0.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 42 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 44 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 46 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.27% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 49 | 0.27% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 50 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 55 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 57 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.24% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.27% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.06% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 15 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.53% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.48% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 27 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |

GC pauses: 0, total 0 ms


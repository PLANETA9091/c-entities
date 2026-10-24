# JVM hotspots (JFR ExecutionSample)

Samples: 81097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 2.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 9 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 18 | 0.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 22 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.48% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 31 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 32 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 37 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 38 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.32% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 41 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 43 | 0.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 44 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 46 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 47 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.24% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 49 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.24% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 55 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 59 | 0.21% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.04% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 12 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 15 | 0.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.59% | `java.util.ArrayList.toArray() line: 370` |
| 18 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.48% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 21 | 0.47% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 23 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.42% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.38% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.36% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.35% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 29 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 30 | 0.34% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms


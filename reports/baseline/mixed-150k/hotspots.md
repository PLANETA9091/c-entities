# JVM hotspots (JFR ExecutionSample)

Samples: 77326

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 3 | 4.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 9 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.88% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 13 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 17 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 18 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 21 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 22 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.34% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 33 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 37 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.28% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 39 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 40 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.27% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 43 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.23% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 48 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 52 | 0.22% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 53 | 0.21% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1465` |
| 57 | 0.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 58 | 0.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 59 | 0.19% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.25% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.14% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.89% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 14 | 0.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 17 | 0.50% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 18 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 19 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 22 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.39% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.tick() line: 872` |
| 27 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 72232

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 21 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.39% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 32 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 34 | 0.36% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 35 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 39 | 0.27% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 41 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 42 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.26% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 46 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.25% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 50 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.22% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 54 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.20% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 57 | 0.20% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.20% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 60 | 0.19% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.03% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.54% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.51% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.47% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 23 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 25 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.39% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 28 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 29 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |

GC pauses: 0, total 0 ms


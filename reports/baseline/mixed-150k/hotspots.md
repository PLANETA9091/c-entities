# JVM hotspots (JFR ExecutionSample)

Samples: 111893

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 7 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 23 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 25 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 30 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 34 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 36 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 37 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.36% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 39 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 40 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 41 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.30% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 46 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 50 | 0.26% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 51 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3785` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 53 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 57 | 0.24% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 59 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 60 | 0.22% | `java.util.Optional.ifPresent(Consumer) line: 178` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 3.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 3.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 13 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.59% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.56% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.47% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 19 | 0.46% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 20 | 0.42% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.39% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 27 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.37% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms


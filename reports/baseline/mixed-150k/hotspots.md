# JVM hotspots (JFR ExecutionSample)

Samples: 105334

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 20 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 32 | 0.43% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 33 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 34 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.31% | `java.util.Map.forEach(BiConsumer) line: 727` |
| 42 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 43 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.26% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 581` |
| 49 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 50 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 52 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 13 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 0.63% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 16 | 0.58% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.50% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.45% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 22 | 0.45% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 25 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.40% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 27 | 0.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 30 | 0.37% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms


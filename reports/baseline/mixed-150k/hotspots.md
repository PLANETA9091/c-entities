# JVM hotspots (JFR ExecutionSample)

Samples: 102964

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 0.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 16 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 18 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 24 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.47% | `java.util.Arrays.fill(Object[], Object) line: 3426` |
| 29 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.40% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 32 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 34 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 35 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 41 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 42 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 44 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 47 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.24% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 51 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 52 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 57 | 0.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 59 | 0.21% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.85% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 14 | 0.67% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.46% | `java.util.EnumMap.clear() line: 359` |
| 20 | 0.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.45% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.42% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.37% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms


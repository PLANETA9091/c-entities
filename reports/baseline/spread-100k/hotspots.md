# JVM hotspots (JFR ExecutionSample)

Samples: 67847

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 1.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.98% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.93% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 24 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.56% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.48% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 34 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 37 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 39 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.44% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 41 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 44 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3785` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 53 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 57 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.29% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 2.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 9 | 1.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 12 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 15 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 19 | 0.70% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.67% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 28 | 0.53% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms


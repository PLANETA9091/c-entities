# JVM hotspots (JFR ExecutionSample)

Samples: 52416

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.12% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 2.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 9 | 1.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 16 | 1.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 1.21% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 23 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 28 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 31 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 36 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.52% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 39 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 40 | 0.49% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 41 | 0.45% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 44 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.41% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 47 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 48 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.38% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 55 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.35% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 59 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 3.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 2.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 11 | 1.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.26% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 1.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 17 | 0.93% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 20 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 23 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


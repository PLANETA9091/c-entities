# JVM hotspots (JFR ExecutionSample)

Samples: 54141

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 1.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 14 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.26% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 19 | 1.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.70% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 32 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 35 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.51% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 42 | 0.47% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 43 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.45% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 47 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 48 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.42% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 50 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 51 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 52 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 56 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.03% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.22% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 1.08% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.76% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 21 | 0.74% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.62% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms


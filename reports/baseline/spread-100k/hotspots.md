# JVM hotspots (JFR ExecutionSample)

Samples: 49439

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 1.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 11 | 1.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 22 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 27 | 0.71% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 31 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 32 | 0.57% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 33 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.52% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 36 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 37 | 0.49% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.47% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 40 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 41 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.46% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 43 | 0.46% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 45 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 47 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 52 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 58 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 5 | 3.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 2.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 2.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 12 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.94% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 0.73% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 21 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 28 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.54% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |

GC pauses: 0, total 0 ms


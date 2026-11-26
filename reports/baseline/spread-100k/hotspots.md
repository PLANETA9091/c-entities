# JVM hotspots (JFR ExecutionSample)

Samples: 53441

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 9 | 1.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 23 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 29 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.50% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.47% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 37 | 0.45% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.42% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 40 | 0.42% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 41 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 42 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 47 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 49 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 53 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.31% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.28% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 60 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.29% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 4 | 2.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 2.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 8 | 2.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 12 | 0.94% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3793` |
| 14 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 0.70% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2207` |
| 26 | 0.49% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 27 | 0.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2818` |
| 29 | 0.47% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 30 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 57109

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 11 | 1.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 13 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 17 | 1.25% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.15% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 28 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 30 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.64% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 35 | 0.57% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 39 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.49% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 41 | 0.48% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.46% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 44 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.44% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 47 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 49 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 53 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 56 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.36% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 58 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 59 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.27% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.74% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 28 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.62% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms


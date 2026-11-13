# JVM hotspots (JFR ExecutionSample)

Samples: 71811

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 3.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 0.91% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 21 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.49% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.49% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 33 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 35 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 37 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.40% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 39 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 41 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.36% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 581` |
| 43 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 49 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 52 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 57 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.25% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 59 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 3.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 11 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 13 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.68% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 16 | 0.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.51% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 21 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.47% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 24 | 0.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 26 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.42% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |

GC pauses: 0, total 0 ms


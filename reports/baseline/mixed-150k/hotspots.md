# JVM hotspots (JFR ExecutionSample)

Samples: 84469

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.45% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 32 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 34 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 36 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 41 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 45 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 47 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 51 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 56 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.22% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.04% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 10 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 0.73% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 19 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.54% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 24 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 25 | 0.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.45% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 27 | 0.44% | `net.minecraft.world.item.ItemStack.isBroken() line: 585` |
| 28 | 0.43% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms


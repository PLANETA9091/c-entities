# JVM hotspots (JFR ExecutionSample)

Samples: 74341

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.53% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.48% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 35 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 36 | 0.47% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 37 | 0.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 40 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 43 | 0.40% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 44 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 49 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.26% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 57 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 59 | 0.24% | `java.util.ArrayDeque.size() line: 647` |
| 60 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.00% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 16 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3443` |
| 17 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 21 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 24 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 25 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.49% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 27 | 0.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 30 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms


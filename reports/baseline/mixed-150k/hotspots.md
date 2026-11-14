# JVM hotspots (JFR ExecutionSample)

Samples: 63938

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.74% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 2.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.73% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 21 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 23 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.56% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.51% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 34 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.39% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 39 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 40 | 0.39% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 41 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.37% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 581` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 45 | 0.35% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 46 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 51 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.30% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 55 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 57 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.27% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 60 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 5.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 0.73% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.51% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 23 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 24 | 0.48% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 27 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 28 | 0.46% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 29 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |

GC pauses: 0, total 0 ms


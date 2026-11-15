# JVM hotspots (JFR ExecutionSample)

Samples: 84844

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 22 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 35 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 36 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 40 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 41 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.35% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 48 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 50 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 52 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 57 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 60 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 20 | 0.50% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 21 | 0.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.45% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 24 | 0.41% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 25 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 26 | 0.40% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 27 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5092` |
| 30 | 0.39% | `java.util.ArrayList.toArray() line: 370` |

GC pauses: 0, total 0 ms


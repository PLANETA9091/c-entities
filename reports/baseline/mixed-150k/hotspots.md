# JVM hotspots (JFR ExecutionSample)

Samples: 86975

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 3.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 11 | 1.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.09% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.61% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 32 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 34 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.34% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 41 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 42 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 45 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 47 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 48 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 51 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.24% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 53 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 59 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.31% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 0.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.44% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.34% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms


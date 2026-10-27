# JVM hotspots (JFR ExecutionSample)

Samples: 61871

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 1.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.95% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.79% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.50% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 38 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.46% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 40 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 43 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap.put(Object, Object) line: 455` |
| 44 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 45 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 49 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.36% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 52 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 53 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 56 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.45% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.96% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.82% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 20 | 0.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |

GC pauses: 0, total 0 ms


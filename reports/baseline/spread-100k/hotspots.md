# JVM hotspots (JFR ExecutionSample)

Samples: 69157

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.51% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.20% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.94% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 27 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 32 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 39 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 41 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 42 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 43 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.37% | `java.util.ImmutableCollections$MapN$MapNIterator.nextIndex() line: 1533` |
| 46 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.36% | `java.util.HashMap.getNode(Object) line: 586` |
| 48 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 49 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 53 | 0.32% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 54 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 57 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 60 | 0.28% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.51% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 15 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.87% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 19 | 0.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 22 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 24 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 25 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms


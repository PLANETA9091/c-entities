# JVM hotspots (JFR ExecutionSample)

Samples: 71435

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.47% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.20% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 17 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.91% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 20 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 22 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.53% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 32 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 36 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 40 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 42 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 43 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 47 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.34% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 49 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 50 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.34% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 54 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 58 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.27% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 1.47% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.38% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 1.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 1.22% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 15 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 19 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 30 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms


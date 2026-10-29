# JVM hotspots (JFR ExecutionSample)

Samples: 91590

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 10 | 1.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.17% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 0.91% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 20 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.53% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 34 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 35 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 38 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 40 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 41 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 44 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 45 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 46 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 47 | 0.32% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 48 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 51 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 52 | 0.28% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 53 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 54 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 58 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 59 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.31% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 11 | 1.21% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.17% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 15 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.85% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 20 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.62% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 22 | 0.59% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 23 | 0.58% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.52% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.47% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms


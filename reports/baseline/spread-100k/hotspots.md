# JVM hotspots (JFR ExecutionSample)

Samples: 70980

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.02% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 14 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.93% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 19 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.73% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 28 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.55% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.49% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 33 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 36 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 37 | 0.40% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 39 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 40 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 41 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.30% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 55 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 56 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 4 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 2.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 7 | 1.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.17% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.93% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 16 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 20 | 0.67% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.61% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.56% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1409` |
| 27 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 28 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 30 | 0.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms


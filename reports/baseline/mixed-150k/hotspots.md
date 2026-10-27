# JVM hotspots (JFR ExecutionSample)

Samples: 120550

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 0.68% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 17 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 23 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 25 | 0.43% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 28 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.35% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 32 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 35 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 41 | 0.27% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 42 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.23% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.21% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 49 | 0.20% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 50 | 0.20% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 54 | 0.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 55 | 0.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 56 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 57 | 0.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.18% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.18% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 41.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.69% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 13 | 0.65% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 19 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.43% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.41% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 24 | 0.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.35% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 26 | 0.35% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.32% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.30% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 121373

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.88% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 15 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.57% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 20 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 26 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.39% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 29 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 32 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 33 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 35 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 37 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 42 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 43 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 47 | 0.25% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 48 | 0.24% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.23% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 52 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 1.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 12 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 15 | 0.54% | `net.minecraft.world.entity.LivingEntity.tick() line: 3444` |
| 16 | 0.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3793` |
| 18 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.44% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.41% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.38% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.37% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 26 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.36% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 30 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms


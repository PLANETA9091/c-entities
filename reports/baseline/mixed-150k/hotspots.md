# JVM hotspots (JFR ExecutionSample)

Samples: 87822

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.62% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 25 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 27 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 29 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 32 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 35 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 38 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 39 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 43 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.27% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 47 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 50 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 53 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.23% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.00% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.63% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.56% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.53% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.43% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

GC pauses: 0, total 0 ms


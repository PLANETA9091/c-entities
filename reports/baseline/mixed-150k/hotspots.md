# JVM hotspots (JFR ExecutionSample)

Samples: 104318

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 31 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 35 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 38 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 39 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 41 | 0.26% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 43 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 44 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.24% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 47 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 49 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.23% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 53 | 0.22% | `java.util.HashMap.getNode(Object) line: 586` |
| 54 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.21% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 57 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.20% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.19% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.19% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.05% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 15 | 0.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.58% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.54% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.48% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.40% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.37% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 27 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.29% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 30 | 0.29% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 117395

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.46% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 27 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 32 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 38 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 39 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 41 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 46 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 47 | 0.26% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 48 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 49 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.24% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 52 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 57 | 0.22% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 58 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.16% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.78% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.48% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.48% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.41% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 25 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 28 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms


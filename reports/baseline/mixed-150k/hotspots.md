# JVM hotspots (JFR ExecutionSample)

Samples: 112245

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 0.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.75% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 30 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 33 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 42 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 45 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.24% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 47 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 48 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.22% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 52 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 53 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 56 | 0.20% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.20% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 58 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.19% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.19% | `net.minecraft.world.entity.Entity.level() line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.51% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.00% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 16 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.49% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 27 | 0.36% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 30 | 0.35% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms


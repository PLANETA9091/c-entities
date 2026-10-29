# JVM hotspots (JFR ExecutionSample)

Samples: 115964

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 22 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 29 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 33 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.29% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 38 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.29% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 40 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 41 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 42 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 43 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 47 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.22% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 54 | 0.21% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.20% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 56 | 0.20% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.20% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 60 | 0.20% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.02% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.44% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 24 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.36% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.34% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


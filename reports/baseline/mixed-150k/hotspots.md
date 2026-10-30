# JVM hotspots (JFR ExecutionSample)

Samples: 103847

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.07% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.43% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 31 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 36 | 0.29% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 37 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.27% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 41 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 43 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 45 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 50 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 54 | 0.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 57 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.19% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.19% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.07% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.52% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.40% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 26 | 0.39% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.37% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 29 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.33% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


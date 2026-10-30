# JVM hotspots (JFR ExecutionSample)

Samples: 144044

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 49.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 8.10% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 4 | 1.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 0.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 14 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 20 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 23 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.34% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 32 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 36 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 37 | 0.22% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 38 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.20% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.20% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 41 | 0.20% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.19% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 43 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.19% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.17% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 46 | 0.17% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 50 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.15% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.15% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 53 | 0.15% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.14% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.14% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 56 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.14% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 59 | 0.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 60 | 0.14% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 49.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 0.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.78% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 9 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.36% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.36% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5087` |
| 21 | 0.33% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.31% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.30% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.27% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 30 | 0.26% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms


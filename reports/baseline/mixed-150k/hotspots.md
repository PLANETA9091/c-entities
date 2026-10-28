# JVM hotspots (JFR ExecutionSample)

Samples: 133131

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.09% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 9 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 11 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 0.75% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 15 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.38% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 29 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.32% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 37 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 38 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 41 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.21% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 46 | 0.20% | `java.util.EnumMap$EnumMapIterator.<init>(EnumMap) line: 515` |
| 47 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.20% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.20% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.20% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.19% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 52 | 0.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.19% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 56 | 0.17% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 57 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.17% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.16% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.97% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.75% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 15 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.46% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.41% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 25 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.31% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.29% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms


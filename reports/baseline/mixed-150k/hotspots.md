# JVM hotspots (JFR ExecutionSample)

Samples: 113009

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 9 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.63% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 35 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 43 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.24% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 50 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 52 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 56 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.20% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 60 | 0.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 18 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.48% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.41% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 27 | 0.37% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms


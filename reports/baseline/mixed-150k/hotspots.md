# JVM hotspots (JFR ExecutionSample)

Samples: 130239

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.00% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.76% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 0.86% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 12 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 31 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 34 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.30% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 37 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 41 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 42 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.25% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.24% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 46 | 0.23% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 47 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 51 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 52 | 0.22% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.21% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.20% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.19% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.level() line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.93% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.86% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 0.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.45% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.34% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.32% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

GC pauses: 0, total 0 ms


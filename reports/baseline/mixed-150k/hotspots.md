# JVM hotspots (JFR ExecutionSample)

Samples: 112521

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 32 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 33 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 37 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 39 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 41 | 0.26% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 42 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 43 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.24% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 45 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.23% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 48 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 52 | 0.22% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 53 | 0.21% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 54 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.20% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.20% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 57 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.19% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 15 | 0.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.46% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.43% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3595` |
| 26 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.33% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.30% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms


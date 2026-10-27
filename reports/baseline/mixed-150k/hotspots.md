# JVM hotspots (JFR ExecutionSample)

Samples: 90259

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 3.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 1.28% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.48% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 33 | 0.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 35 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 36 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 41 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 44 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 51 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 56 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.32% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.29% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 15 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.44% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 24 | 0.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 25 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.36% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms


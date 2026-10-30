# JVM hotspots (JFR ExecutionSample)

Samples: 120006

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 31 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.33% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 41 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 43 | 0.25% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 44 | 0.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 45 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 46 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 47 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 48 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 49 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.21% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 52 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.20% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.20% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 56 | 0.20% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 57 | 0.19% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.18% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.04% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.63% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.52% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.48% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.41% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.36% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 28 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


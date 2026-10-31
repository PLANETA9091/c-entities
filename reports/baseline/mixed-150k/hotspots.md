# JVM hotspots (JFR ExecutionSample)

Samples: 90423

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 9 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.40% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 19 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.48% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 31 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.42% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 36 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 37 | 0.35% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 38 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 42 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 48 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 49 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.21% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 59 | 0.20% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.20% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.70% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.24% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 0.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.84% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 19 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 24 | 0.43% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.42% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.40% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.35% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms


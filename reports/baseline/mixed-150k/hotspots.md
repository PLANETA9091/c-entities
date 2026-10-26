# JVM hotspots (JFR ExecutionSample)

Samples: 83681

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.62% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 22 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 26 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 27 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.44% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 30 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 35 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.33% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 37 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 39 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 46 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 47 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 48 | 0.27% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.25% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 53 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 56 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 60 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.00% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 11 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 13 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.71% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 16 | 0.61% | `java.util.ArrayList.toArray() line: 370` |
| 17 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.52% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.50% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 26 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.38% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 28 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


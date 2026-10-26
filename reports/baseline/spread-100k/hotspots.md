# JVM hotspots (JFR ExecutionSample)

Samples: 67746

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 11 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.85% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.72% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.67% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 27 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 36 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 38 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 42 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.39% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.34% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 54 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 56 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 58 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 59 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 9 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.22% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 13 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 0.83% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 23 | 0.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.67% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 26 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 28 | 0.59% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |

GC pauses: 0, total 0 ms


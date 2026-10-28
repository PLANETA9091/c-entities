# JVM hotspots (JFR ExecutionSample)

Samples: 75074

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.96% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.77% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 23 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.60% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 31 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 33 | 0.52% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 34 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 35 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 37 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 41 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 42 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 48 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.34% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 50 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 55 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 56 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.17% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 14 | 1.00% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 0.91% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.77% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 22 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.59% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 29 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms


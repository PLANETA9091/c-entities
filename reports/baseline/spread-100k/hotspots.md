# JVM hotspots (JFR ExecutionSample)

Samples: 71176

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.15% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.75% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 22 | 0.72% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.72% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 24 | 0.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 31 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.42% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 44 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 45 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 49 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 52 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 53 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 54 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.28% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.06% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.15% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.82% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 16 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 19 | 0.70% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 20 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 24 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.61% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.54% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms


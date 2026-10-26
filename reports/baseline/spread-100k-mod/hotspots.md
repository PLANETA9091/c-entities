# JVM hotspots (JFR ExecutionSample)

Samples: 30774

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 2 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 4 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 7 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.91% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.87% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 20 | 0.86% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 21 | 0.83% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.72% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.68% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.61% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2107` |
| 38 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 40 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4378` |
| 43 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 50 | 0.40% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 52 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4723` |
| 58 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 60 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 7 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.94% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 11 | 1.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4733` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 16 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 19 | 1.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.88% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.81% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4665` |
| 29 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms


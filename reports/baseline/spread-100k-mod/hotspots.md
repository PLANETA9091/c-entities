# JVM hotspots (JFR ExecutionSample)

Samples: 30344

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6923` |
| 3 | 3.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 3.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 6 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 9 | 1.31% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6759` |
| 10 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.96% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 0.89% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.86% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 22 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.83% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.75% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 32 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 34 | 0.56% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.56% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 36 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 38 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 40 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4208` |
| 42 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 49 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.37% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 51 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.35% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 55 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 60 | 0.30% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 3.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 7 | 2.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 8 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 13 | 1.16% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 18 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.97% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 25 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.74% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.73% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms


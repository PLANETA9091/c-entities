# JVM hotspots (JFR ExecutionSample)

Samples: 31917

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 2 | 3.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 3 | 3.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 3.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.05% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 12 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.03% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.91% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.80% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 17 | 0.79% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 18 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.76% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4527` |
| 30 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4700` |
| 36 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 38 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.39% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 50 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 56 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 59 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 8 | 2.05% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 11 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.49% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.33% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 17 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4987` |
| 21 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 25 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 30629

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 4 | 3.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 7 | 2.26% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 12 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 17 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.84% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.82% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 20 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 27 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 31 | 0.58% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 32 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 35 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4700` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.45% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 42 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 44 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.38% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 51 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 52 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 53 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 55 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 57 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 58 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 6 | 2.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.26% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 9 | 1.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.52% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 13 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 15 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 19 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.82% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 21 | 0.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4987` |
| 24 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.69% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5416` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 31271

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.43% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 4 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 6 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 11 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5326` |
| 16 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 4887` |
| 27 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3974` |
| 31 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.48% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 37 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4070` |
| 40 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 44 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.42% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4146` |
| 47 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 49 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 53 | 0.38% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 54 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 56 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 58 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 60 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.26% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 4.24% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 4 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.85% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 6 | 2.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3808` |
| 12 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 18 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4158` |
| 20 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.67% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 26 | 0.66% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 27 | 0.63% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 28 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 36566

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6921` |
| 4 | 1.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 5 | 1.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 7 | 1.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6757` |
| 11 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.29% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 1.05% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 17 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 20 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 26 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.56% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 33 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.46% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 39 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 41 | 0.41% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 46 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5127` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 51 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 53 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 57 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 60 | 0.31% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4415` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 3 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.64% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 5 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 6 | 2.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.83% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 12 | 1.38% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 14 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 19 | 1.10% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 1.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.98% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5006` |
| 28 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms


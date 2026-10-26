# JVM hotspots (JFR ExecutionSample)

Samples: 36700

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 4 | 1.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 5 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.53% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 11 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 17 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 22 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 24 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 26 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.52% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 35 | 0.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 36 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 37 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 39 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 40 | 0.41% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 45 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 47 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 48 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 54 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 56 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.31% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 58 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2562` |
| 60 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 3 | 3.01% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.81% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 5 | 2.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 6 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 9 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 11 | 1.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 13 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 17 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.87% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5872` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5870` |
| 25 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.72% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5003` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms


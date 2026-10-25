# JVM hotspots (JFR ExecutionSample)

Samples: 26015

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 4 | 2.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 5 | 2.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 9 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 11 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.15% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.05% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5343` |
| 15 | 1.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 18 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 20 | 0.81% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2097` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 25 | 0.76% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 32 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3822` |
| 33 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.60% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 35 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.49% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 40 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 41 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.44% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 48 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2022` |
| 51 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.39% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 55 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4081` |
| 57 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.02% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3819` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 8 | 2.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 12 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4199` |
| 17 | 1.11% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 20 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 25 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 27 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 29 | 0.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |

GC pauses: 0, total 0 ms


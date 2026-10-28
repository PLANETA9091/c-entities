# JVM hotspots (JFR ExecutionSample)

Samples: 29631

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 2 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 4 | 3.46% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 5 | 2.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 10 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4161` |
| 16 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 1.00% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.83% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 28 | 0.73% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 29 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 31 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.60% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 36 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 37 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2856` |
| 41 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 45 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4884` |
| 49 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 50 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 53 | 0.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7905` |
| 55 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.33% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7815` |
| 6 | 2.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.23% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.69% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 11 | 1.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4198` |
| 15 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 18 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5773` |
| 27 | 0.77% | `net.minecraft.world.entity.Mob.inactiveTick() line: 229` |
| 28 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4881` |
| 30 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |

GC pauses: 0, total 0 ms


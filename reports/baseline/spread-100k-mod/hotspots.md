# JVM hotspots (JFR ExecutionSample)

Samples: 34565

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8102` |
| 2 | 3.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8243` |
| 4 | 3.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 3.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5138` |
| 7 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4420` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8049` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4993` |
| 12 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.97% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.84% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.71% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.65% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3115` |
| 30 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8188` |
| 32 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.49% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 39 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8063` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.47% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 45 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 51 | 0.39% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 53 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 55 | 0.35% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 523` |
| 56 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 57 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 58 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8098` |
| 8 | 2.23% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8115` |
| 11 | 1.43% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8090` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4457` |
| 14 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1732` |
| 16 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.73% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.72% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3028` |
| 28 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms


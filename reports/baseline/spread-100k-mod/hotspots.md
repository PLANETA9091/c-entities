# JVM hotspots (JFR ExecutionSample)

Samples: 32281

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 3 | 3.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 5 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 9 | 1.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 12 | 1.29% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.88% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.85% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 20 | 0.84% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.55% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 39 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7797` |
| 41 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 43 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 44 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 47 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 50 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 54 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 55 | 0.31% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 56 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.23% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7849` |
| 12 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.29% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 16 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.86% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.84% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 27 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2786` |

GC pauses: 0, total 0 ms


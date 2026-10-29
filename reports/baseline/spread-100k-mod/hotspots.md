# JVM hotspots (JFR ExecutionSample)

Samples: 32404

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8209` |
| 3 | 3.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 5 | 2.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5104` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8068` |
| 9 | 1.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4386` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 1.18% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4959` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.03% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 21 | 0.77% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 22 | 0.73% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 23 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2194` |
| 25 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5106` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3081` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 34 | 0.55% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 37 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.53% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 39 | 0.50% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 40 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 41 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 44 | 0.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 45 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 47 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8154` |
| 49 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.38% | `java.lang.String.hashCode() line: 2433` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2213` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 56 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 57 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2973` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 7 | 2.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1713` |
| 11 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4423` |
| 13 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.19% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 18 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8081` |
| 20 | 0.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.79% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 26 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5106` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2994` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |

GC pauses: 0, total 0 ms


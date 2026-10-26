# JVM hotspots (JFR ExecutionSample)

Samples: 30232

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 5 | 2.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3860` |
| 7 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 12 | 1.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3888` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5550` |
| 14 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 20 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.75% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4192` |
| 25 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.65% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.56% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4029` |
| 35 | 0.53% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.47% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 44 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.44% | `java.lang.String.hashCode() line: 2433` |
| 48 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4288` |
| 53 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 55 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 58 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.33% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.81% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.71% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 6 | 2.70% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 7 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 9 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.49% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4026` |
| 13 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4376` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 19 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 26 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms


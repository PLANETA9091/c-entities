# JVM hotspots (JFR ExecutionSample)

Samples: 30636

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 3 | 2.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3627` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5454` |
| 9 | 1.58% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 10 | 1.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5317` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3655` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.88% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.84% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.78% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |
| 30 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.65% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3959` |
| 33 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3796` |
| 39 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.49% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 46 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 49 | 0.41% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 50 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.39% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3948` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4055` |
| 59 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.84% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.84% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 7 | 2.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 11 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3793` |
| 13 | 1.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 1.27% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4143` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 18 | 1.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 20 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 1.03% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 25 | 0.79% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 26 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 29 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |

GC pauses: 0, total 0 ms


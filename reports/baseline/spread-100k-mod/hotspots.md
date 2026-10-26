# JVM hotspots (JFR ExecutionSample)

Samples: 29837

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 5 | 2.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 8 | 2.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.97% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 19 | 0.88% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 21 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 24 | 0.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.81% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.73% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 33 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.65% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 35 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 37 | 0.61% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.55% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 40 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.54% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 42 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 44 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 45 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 47 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 49 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 52 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 53 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 55 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 57 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 6 | 2.53% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 7 | 2.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 8 | 2.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 2.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 11 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 14 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 18 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 19 | 1.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 1.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 22 | 1.14% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 23 | 0.99% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 29 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 25754

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.12% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6660` |
| 4 | 2.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 5 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 9 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 10 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6496` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.90% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 21 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.75% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 25 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.59% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 32 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.56% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 34 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 36 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4725` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 38 | 0.52% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 39 | 0.52% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 40 | 0.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 43 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 52 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 54 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 59 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 4 | 3.13% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 6 | 2.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 7 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 13 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4667` |
| 19 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 1.09% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 1.06% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 24 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4735` |
| 25 | 1.03% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 26 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.92% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4725` |
| 29 | 0.80% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5005` |
| 30 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


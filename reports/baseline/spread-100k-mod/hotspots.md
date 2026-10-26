# JVM hotspots (JFR ExecutionSample)

Samples: 29154

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 5 | 2.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6921` |
| 6 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 9 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.26% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6757` |
| 11 | 1.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 15 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 17 | 0.86% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 25 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 28 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.61% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 32 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 38 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 39 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 41 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 47 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4669` |
| 50 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 51 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 52 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 53 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 58 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.99% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 3 | 2.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 5 | 2.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 10 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 12 | 1.29% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 14 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 15 | 1.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 20 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4736` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 27 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 30 | 0.66% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms


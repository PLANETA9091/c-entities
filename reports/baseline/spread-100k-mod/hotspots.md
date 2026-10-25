# JVM hotspots (JFR ExecutionSample)

Samples: 34036

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3188` |
| 2 | 3.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 4 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3844` |
| 6 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 8 | 1.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.12% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.97% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 19 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 21 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 23 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 27 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.70% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.68% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 31 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 33 | 0.63% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.62% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3345` |
| 35 | 0.61% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.55% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 39 | 0.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2079` |
| 40 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 43 | 0.46% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 44 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 46 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 48 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 51 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1394` |
| 53 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 55 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 56 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.33% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3441` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.32% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.12% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2296` |
| 5 | 2.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.51% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 8 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2692` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 15 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.15% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 19 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3529` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 24 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 26 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 28 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.gateCell(Level, AABB) line: 937` |

GC pauses: 0, total 0 ms


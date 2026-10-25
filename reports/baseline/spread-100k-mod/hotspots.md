# JVM hotspots (JFR ExecutionSample)

Samples: 46836

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.47% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 4 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 6 | 1.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3467` |
| 8 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.14% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3477` |
| 14 | 0.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4363` |
| 17 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4277` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 21 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |
| 23 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 27 | 0.60% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.60% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.58% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3578` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3781` |
| 38 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 39 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 40 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 42 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 48 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.40% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 50 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1405` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1408` |
| 52 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 53 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3618` |
| 55 | 0.36% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1424` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 60 | 0.33% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 2.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.83% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2647` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 602` |
| 9 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1058` |
| 11 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.13% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3615` |
| 15 | 0.96% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3965` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 20 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 23 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 26 | 0.74% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 27360

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 3 | 3.54% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 4 | 3.44% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 5 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 7 | 1.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 9 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.99% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |
| 16 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4124` |
| 18 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.82% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.77% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 24 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2084` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.67% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 29 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 31 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 36 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 38 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2058` |
| 40 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1404` |
| 41 | 0.52% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3628` |
| 43 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 46 | 0.48% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 51 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 52 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3724` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2020` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.22% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 4 | 3.43% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 601` |
| 5 | 2.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3462` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2494` |
| 8 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 11 | 1.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1057` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3842` |
| 15 | 1.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.05% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 22 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 24 | 0.80% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 26 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3812` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 30 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


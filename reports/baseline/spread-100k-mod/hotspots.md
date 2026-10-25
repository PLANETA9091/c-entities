# JVM hotspots (JFR ExecutionSample)

Samples: 27623

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.48% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4065` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3138` |
| 4 | 3.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1493` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3284` |
| 8 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 2.16% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4034` |
| 10 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3148` |
| 14 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 3948` |
| 17 | 1.15% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 20 | 1.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2107` |
| 21 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.80% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 25 | 0.80% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.70% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3452` |
| 32 | 0.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.63% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 36 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 37 | 0.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.56% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 41 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 46 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 47 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 48 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 49 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2010` |
| 51 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 52 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3289` |
| 53 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 54 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1422` |
| 55 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.66% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.01% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2355` |
| 5 | 2.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.53% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 600` |
| 7 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 1.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.52% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.51% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1056` |
| 16 | 1.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 19 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3624` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 21 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2031` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3636` |
| 23 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3286` |
| 25 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 27 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3666` |
| 29 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms


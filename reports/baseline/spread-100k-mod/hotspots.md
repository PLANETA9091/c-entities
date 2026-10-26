# JVM hotspots (JFR ExecutionSample)

Samples: 29544

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 5 | 2.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6507` |
| 6 | 2.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 9 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.26% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6343` |
| 12 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 15 | 0.93% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 16 | 0.91% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 17 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 21 | 0.78% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 40 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.45% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 42 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 43 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 44 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 45 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 51 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4365` |
| 54 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.33% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 58 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.31% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 60 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.57% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 5 | 2.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4003` |
| 6 | 2.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 9 | 1.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 14 | 1.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 1.23% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 24 | 0.91% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 25 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 26 | 0.84% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5001` |
| 30 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


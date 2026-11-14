# JVM hotspots (JFR ExecutionSample)

Samples: 26355

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.80% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 5.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3118` |
| 3 | 4.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3119` |
| 4 | 1.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 0.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12445` |
| 8 | 0.86% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 9 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 11 | 0.57% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 12 | 0.57% | `java.util.ArrayList.add(Object) line: 497` |
| 13 | 0.54% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.54% | `java.lang.ThreadLocal.get(Thread) line: 185` |
| 15 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 16 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8486` |
| 18 | 0.51% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 19 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3186` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8317` |
| 24 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 25 | 0.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 26 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8139` |
| 28 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 29 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.38% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 32 | 0.35% | `java.util.ArrayList.elementData(int) line: 412` |
| 33 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6004` |
| 36 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 37 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5615` |
| 39 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 40 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 45 | 0.27% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 46 | 0.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 47 | 0.27% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4584` |
| 49 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 52 | 0.24% | `java.util.EnumMap.get(Object) line: 248` |
| 53 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3101` |
| 55 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 58 | 0.22% | `java.util.ArrayList.add(Object) line: 496` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5924` |
| 60 | 0.22% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.48% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 11.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 3 | 1.83% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 1.79% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 7 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 0.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12669` |
| 10 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 11 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 0.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3187` |
| 13 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 0.61% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8579` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8485` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8564` |
| 21 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.55% | `java.lang.ThreadLocal.get() line: 171` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8655` |
| 25 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.47% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 27 | 0.47% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2921` |

GC pauses: 0, total 0 ms


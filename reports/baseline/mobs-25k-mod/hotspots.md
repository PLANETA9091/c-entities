# JVM hotspots (JFR ExecutionSample)

Samples: 31574

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.50% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 4.98% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 3 | 1.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 4 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 8 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12511` |
| 9 | 0.58% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 0.51% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3126` |
| 17 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 19 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.42% | `java.util.ArrayList.add(Object) line: 497` |
| 21 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8552` |
| 22 | 0.41% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 23 | 0.39% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8205` |
| 24 | 0.38% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8383` |
| 26 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5651` |
| 29 | 0.33% | `java.util.ArrayList.elementData(int) line: 412` |
| 30 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9175` |
| 32 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.28% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 37 | 0.27% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 38 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3107` |
| 39 | 0.27% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5714` |
| 41 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 43 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.24% | `java.util.ArrayList.add(Object) line: 496` |
| 45 | 0.24% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13100` |
| 47 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6040` |
| 49 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5960` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3140` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8232` |
| 53 | 0.20% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 54 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 57 | 0.18% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.18% | `java.util.Collections$UnmodifiableCollection$1.hasNext() line: 1079` |
| 59 | 0.18% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 5.45% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 4 | 1.42% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.27% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 7 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 10 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12735` |
| 12 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 16 | 0.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8630` |
| 18 | 0.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8551` |
| 20 | 0.54% | `java.util.HashMap.get(Object) line: 565` |
| 21 | 0.54% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8645` |
| 24 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8721` |
| 28 | 0.41% | `java.lang.ThreadLocal.get() line: 171` |
| 29 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 34217

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 43.31% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 6.06% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 3 | 1.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 4 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 0.70% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 6 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 7 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 9 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 16 | 0.38% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 17 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8829` |
| 18 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 21 | 0.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.34% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8660` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8482` |
| 28 | 0.29% | `java.util.ArrayList.add(Object) line: 497` |
| 29 | 0.28% | `java.util.ArrayList.add(Object) line: 496` |
| 30 | 0.28% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 31 | 0.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 32 | 0.26% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.26% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 34 | 0.25% | `java.util.ArrayList.elementData(int) line: 412` |
| 35 | 0.25% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5839` |
| 37 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 38 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9452` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3133` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6228` |
| 42 | 0.21% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3157` |
| 44 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 46 | 0.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6148` |
| 48 | 0.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.19% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5902` |
| 51 | 0.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 52 | 0.18% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13570` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 13751` |
| 57 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8509` |
| 59 | 0.17% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5814` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 43.31% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 6.51% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 4 | 1.39% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.24% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 8 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 0.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 10 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 12 | 0.57% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 13 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |
| 14 | 0.53% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1402` |
| 15 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 0.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8922` |
| 19 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8907` |
| 20 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.44% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8998` |
| 24 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8828` |
| 27 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.36% | `java.lang.ThreadLocal.get() line: 171` |
| 29 | 0.35% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


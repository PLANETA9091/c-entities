# JVM hotspots (JFR ExecutionSample)

Samples: 21083

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.29% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 3.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 3 | 2.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3126` |
| 4 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.07% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 6 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12887` |
| 9 | 0.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 10 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 13 | 0.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.66% | `java.util.ArrayList.add(Object) line: 497` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8763` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 20 | 0.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 22 | 0.52% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 23 | 0.52% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.50% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8416` |
| 25 | 0.50% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8594` |
| 28 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.44% | `java.util.ArrayList.elementData(int) line: 412` |
| 31 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 33 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6206` |
| 35 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 36 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 38 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 39 | 0.35% | `java.util.ArrayList.grow() line: 245` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5817` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5880` |
| 42 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 43 | 0.32% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 45 | 0.31% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9386` |
| 46 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3140` |
| 48 | 0.29% | `java.util.ArrayList.add(Object) line: 496` |
| 49 | 0.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.28% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6209` |
| 54 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.27% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 56 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8443` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 60 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.01% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 8.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2889` |
| 3 | 2.40% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.09% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13129` |
| 7 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 8 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 10 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 13 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.77% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8856` |
| 18 | 0.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8841` |
| 20 | 0.69% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.68% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8762` |
| 25 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8932` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 27 | 0.55% | `java.lang.ThreadLocal.get() line: 171` |
| 28 | 0.54% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.52% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


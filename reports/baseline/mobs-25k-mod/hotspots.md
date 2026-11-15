# JVM hotspots (JFR ExecutionSample)

Samples: 20492

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.04% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 4.23% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 3.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 5 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.01% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12953` |
| 12 | 0.91% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 13 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8829` |
| 15 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 20 | 0.58% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.56% | `java.util.ArrayList.add(Object) line: 497` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8482` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3210` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 26 | 0.49% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 27 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 28 | 0.47% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2947` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8660` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5839` |
| 32 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3038` |
| 36 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9452` |
| 38 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 39 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.37% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 41 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 42 | 0.36% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 43 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 45 | 0.35% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 46 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 47 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6228` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5814` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6148` |
| 55 | 0.28% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.28% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5902` |
| 59 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.27% | `java.util.ArrayList.grow() line: 245` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.74% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 9.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 3 | 4.23% | `java.util.ArrayList.get(int) line: 429` |
| 4 | 2.41% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 2.15% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 10 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 11 | 1.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8922` |
| 13 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8828` |
| 19 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |
| 21 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8907` |
| 24 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.65% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1402` |
| 27 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8998` |
| 29 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.51% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


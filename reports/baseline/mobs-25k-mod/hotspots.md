# JVM hotspots (JFR ExecutionSample)

Samples: 57229

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 56.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4012` |
| 2 | 6.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4084` |
| 3 | 2.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3864` |
| 4 | 1.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4026` |
| 5 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3995` |
| 6 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 7 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4011` |
| 8 | 0.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3994` |
| 10 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3863` |
| 12 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.35% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 14 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3993` |
| 20 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11450` |
| 21 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4010` |
| 22 | 0.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 23 | 0.25% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 24 | 0.23% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11103` |
| 25 | 0.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 26 | 0.22% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2114` |
| 27 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.19% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.18% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.17% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11281` |
| 31 | 0.17% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 32 | 0.17% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7291` |
| 33 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.15% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.14% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 37 | 0.14% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 12073` |
| 38 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.13% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.13% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 42 | 0.13% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7354` |
| 43 | 0.13% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 45 | 0.13% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.13% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 47 | 0.13% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 48 | 0.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 49 | 0.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11528` |
| 50 | 0.12% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 17101` |
| 51 | 0.12% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 52 | 0.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4044` |
| 53 | 0.11% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7266` |
| 54 | 0.11% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.11% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3597` |
| 56 | 0.11% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.10% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.10% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 59 | 0.10% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 60 | 0.10% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 70.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2114` |
| 2 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 3 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 5 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 7 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 8 | 0.43% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11543` |
| 11 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11449` |
| 13 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16288` |
| 14 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11528` |
| 15 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 16 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.29% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.25% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 21 | 0.24% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 22 | 0.24% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 23 | 0.23% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11619` |
| 24 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.21% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.20% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.17% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 20585

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.74% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 2.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3075` |
| 3 | 2.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3074` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5876` |
| 5 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 8 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.86% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 0.83% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 13 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 16 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8304` |
| 18 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.67% | `java.util.ArrayList.add(Object) line: 497` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2970` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2879` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 27 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 28 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7957` |
| 30 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.50% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 32 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8135` |
| 34 | 0.47% | `java.util.ArrayList.elementData(int) line: 412` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3057` |
| 37 | 0.45% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 38 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12852` |
| 39 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 40 | 0.40% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5459` |
| 42 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 45 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.31% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5434` |
| 51 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7984` |
| 53 | 0.30% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 54 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.29% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 57 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3089` |
| 59 | 0.28% | `java.util.ArrayList.add(Object) line: 496` |
| 60 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.45% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 8.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 3 | 3.53% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.14% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 7 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 8 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 10 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8382` |
| 11 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8397` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.85% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8303` |
| 22 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8473` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map) line: 134` |
| 27 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms


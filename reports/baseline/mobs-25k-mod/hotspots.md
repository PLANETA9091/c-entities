# JVM hotspots (JFR ExecutionSample)

Samples: 50601

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 59.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4020` |
| 2 | 5.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4092` |
| 3 | 1.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4034` |
| 4 | 1.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4010` |
| 5 | 1.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4019` |
| 6 | 1.02% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 7 | 0.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4002` |
| 8 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 0.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4018` |
| 11 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 14 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2119` |
| 16 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 18 | 0.26% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 19 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 20 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11485` |
| 21 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.22% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11138` |
| 24 | 0.22% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 25 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 26 | 0.22% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11316` |
| 27 | 0.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 28 | 0.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11563` |
| 29 | 0.20% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 30 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.17% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 11165` |
| 32 | 0.16% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7718` |
| 33 | 0.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 34 | 0.16% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 35 | 0.16% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 36 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10877` |
| 37 | 0.15% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.15% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 39 | 0.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 40 | 0.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.14% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 43 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 44 | 0.13% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7389` |
| 45 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 46 | 0.13% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 47 | 0.12% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 48 | 0.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4001` |
| 49 | 0.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 50 | 0.12% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5553` |
| 51 | 0.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 52 | 0.11% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7301` |
| 53 | 0.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4052` |
| 54 | 0.11% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 55 | 0.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 57 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7326` |
| 58 | 0.11% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 59 | 0.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.11% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 70.68% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2119` |
| 2 | 1.01% | `java.util.ArrayList.grow(int) line: 238` |
| 3 | 0.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 6 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11578` |
| 7 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 8 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 10 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11563` |
| 13 | 0.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11484` |
| 14 | 0.30% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 15 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 16 | 0.29% | `java.util.HashMap.get(Object) line: 565` |
| 17 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.27% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 19 | 0.27% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11654` |
| 21 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.25% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.24% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.24% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11300` |
| 25 | 0.22% | `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal.findTarget() line: 66` |
| 26 | 0.21% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 27 | 0.20% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 28 | 0.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 29 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


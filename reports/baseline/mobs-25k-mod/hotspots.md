# JVM hotspots (JFR ExecutionSample)

Samples: 27177

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.79% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 4.23% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 3 | 1.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3362` |
| 4 | 1.02% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3126` |
| 7 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 0.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.80% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1395` |
| 10 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.61% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 14 | 0.58% | `java.util.ArrayList.elementData(int) line: 412` |
| 15 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12804` |
| 16 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 18 | 0.54% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8836` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8489` |
| 21 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 23 | 0.48% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 24 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3258` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8914` |
| 27 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 28 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.37% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 31 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 32 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 33 | 0.34% | `java.util.ArrayList.add(Object) line: 496` |
| 34 | 0.33% | `java.util.ArrayList.grow() line: 245` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3344` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 37 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 38 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8292` |
| 39 | 0.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 40 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3430` |
| 41 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8667` |
| 43 | 0.27% | `net.minecraft.world.entity.ai.control.LookControl.setLookAt(double, double, double) line: 35` |
| 44 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3353` |
| 45 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 47 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9459` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8516` |
| 50 | 0.24% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3377` |
| 55 | 0.22% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.22% | `java.lang.ThreadLocal.get(Thread) line: 185` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6307` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5981` |
| 59 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.21% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.79% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 5.44% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3126` |
| 4 | 1.53% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.07% | `java.util.ArrayList.add(Object) line: 497` |
| 7 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8929` |
| 9 | 1.00% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1386` |
| 10 | 0.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8835` |
| 14 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13032` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.60% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.58% | `java.util.ArrayList.get(int) line: 429` |
| 20 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.54% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.54% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 25 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.49% | `java.util.HashMap.get(Object) line: 565` |
| 29 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.48% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 46661

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 55.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3797` |
| 2 | 5.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3869` |
| 3 | 1.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3811` |
| 4 | 1.12% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 0.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3795` |
| 6 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3779` |
| 7 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3780` |
| 8 | 0.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3796` |
| 9 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 14 | 0.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3787` |
| 16 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10559` |
| 17 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.32% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.32% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 20 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.30% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 23 | 0.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10390` |
| 27 | 0.26% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10212` |
| 28 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10637` |
| 30 | 0.24% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 31 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3778` |
| 32 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15127` |
| 33 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.20% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 35 | 0.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3829` |
| 36 | 0.20% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10239` |
| 37 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 38 | 0.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3649` |
| 39 | 0.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 40 | 0.18% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.18% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 42 | 0.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 43 | 0.17% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 44 | 0.17% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.16% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 46 | 0.16% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7132` |
| 47 | 0.16% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.16% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 49 | 0.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.15% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 51 | 0.15% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11182` |
| 52 | 0.15% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1926` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7135` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6743` |
| 56 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 57 | 0.14% | `java.util.EnumMap.get(Object) line: 248` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6806` |
| 59 | 0.13% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 66.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1926` |
| 2 | 1.12% | `java.util.ArrayList.add(Object) line: 497` |
| 3 | 0.98% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10652` |
| 6 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.36% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 15 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10728` |
| 16 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10558` |
| 17 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10637` |
| 18 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 19 | 0.33% | `java.util.HashMap.get(Object) line: 565` |
| 20 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.30% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 23 | 0.28% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10374` |
| 24 | 0.28% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15404` |
| 26 | 0.26% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.24% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 28 | 0.23% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.21% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |

GC pauses: 0, total 0 ms


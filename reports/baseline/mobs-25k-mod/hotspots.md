# JVM hotspots (JFR ExecutionSample)

Samples: 22283

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.01% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1836` |
| 2 | 4.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3680` |
| 3 | 3.71% | `java.util.ArrayList.elementData(int) line: 412` |
| 4 | 3.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3679` |
| 5 | 2.74% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 6 | 2.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 7 | 1.67% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 1.52% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 9 | 1.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3747` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3400` |
| 11 | 1.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.11% | `java.util.ArrayList.grow() line: 245` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1837` |
| 14 | 0.96% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 16 | 0.77% | `java.lang.ThreadLocal.get(Thread) line: 185` |
| 17 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.60% | `java.util.ArrayList.add(Object) line: 496` |
| 24 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 25 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9931` |
| 27 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3694` |
| 28 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3662` |
| 30 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9584` |
| 32 | 0.44% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9762` |
| 33 | 0.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6554` |
| 35 | 0.37% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.36% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13654` |
| 37 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3663` |
| 39 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 40 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6491` |
| 42 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 43 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 44 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 45 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.28% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10554` |
| 48 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 49 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 52 | 0.25% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.25% | `java.util.EnumMap.get(Object) line: 248` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 55 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6466` |
| 57 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 60 | 0.23% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9611` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.20% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 11.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3400` |
| 3 | 5.22% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 3.71% | `java.util.ArrayList.get(int) line: 429` |
| 5 | 2.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3748` |
| 6 | 2.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 7 | 1.30% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1828` |
| 8 | 1.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.11% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 11 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 13 | 0.93% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10024` |
| 16 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 17 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.78% | `java.lang.ThreadLocal.get() line: 171` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10009` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9930` |
| 24 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.58% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10100` |
| 27 | 0.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.49% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


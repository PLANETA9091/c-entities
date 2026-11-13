# JVM hotspots (JFR ExecutionSample)

Samples: 86505

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 44.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 8.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 3 | 4.33% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 3.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2882` |
| 5 | 2.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 6 | 1.85% | `java.util.ArrayList.elementData(int) line: 412` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 8 | 1.35% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 9 | 0.93% | `java.util.ArrayList.add(Object) line: 497` |
| 10 | 0.89% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 11 | 0.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 12 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 13 | 0.72% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2864` |
| 15 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |
| 16 | 0.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2949` |
| 17 | 0.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2872` |
| 18 | 0.58% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 19 | 0.56% | `java.util.ArrayList.grow() line: 245` |
| 20 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2865` |
| 22 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2880` |
| 26 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2863` |
| 27 | 0.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7891` |
| 31 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 33 | 0.22% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7722` |
| 34 | 0.22% | `java.util.HashMap.getNode(Object) line: 586` |
| 35 | 0.22% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7544` |
| 36 | 0.20% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 37 | 0.20% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7969` |
| 39 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 40 | 0.19% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7571` |
| 41 | 0.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 42 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 43 | 0.15% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5268` |
| 44 | 0.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 45 | 0.15% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5205` |
| 46 | 0.14% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 47 | 0.14% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 48 | 0.13% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 49 | 0.13% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 50 | 0.13% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 51 | 0.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2777` |
| 52 | 0.12% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 53 | 0.12% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.12% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1379` |
| 55 | 0.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2914` |
| 56 | 0.11% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.11% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 60 | 0.11% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 47.57% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 16.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 3 | 5.79% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1369` |
| 5 | 1.85% | `java.util.ArrayList.get(int) line: 429` |
| 6 | 1.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2950` |
| 7 | 1.35% | `java.lang.ThreadLocal.get() line: 171` |
| 8 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 9 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 11 | 0.56% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 12 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7984` |
| 13 | 0.47% | `java.util.ArrayList.grow(int) line: 238` |
| 14 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 0.36% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 0.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7969` |
| 21 | 0.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7890` |
| 22 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8060` |
| 23 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 24 | 0.24% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7706` |
| 25 | 0.24% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.24% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 27 | 0.24% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.21% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


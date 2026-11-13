# JVM hotspots (JFR ExecutionSample)

Samples: 66616

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.81% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 8.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2895` |
| 3 | 7.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2894` |
| 4 | 4.70% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 6 | 3.24% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 7 | 2.83% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2962` |
| 9 | 2.05% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 10 | 1.84% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 12 | 1.45% | `java.util.ArrayList.elementData(int) line: 412` |
| 13 | 1.14% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 1.12% | `java.util.ArrayList.grow() line: 245` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 16 | 0.74% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 17 | 0.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2909` |
| 18 | 0.60% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 19 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2878` |
| 22 | 0.26% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 23 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2893` |
| 24 | 0.24% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 26 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7904` |
| 28 | 0.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2885` |
| 29 | 0.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11862` |
| 31 | 0.19% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.18% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7557` |
| 34 | 0.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 35 | 0.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 36 | 0.14% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5281` |
| 37 | 0.14% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 38 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.14% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7735` |
| 40 | 0.13% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5218` |
| 41 | 0.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 42 | 0.12% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8527` |
| 43 | 0.11% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 44 | 0.11% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2790` |
| 47 | 0.11% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 48 | 0.11% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.10% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 50 | 0.10% | `java.util.HashMap.getNode(Object) line: 586` |
| 51 | 0.10% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7982` |
| 52 | 0.10% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5635` |
| 53 | 0.10% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 54 | 0.10% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.09% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.09% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2876` |
| 59 | 0.08% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 60 | 0.08% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.16% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 20.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2658` |
| 3 | 9.78% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 3.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2963` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1369` |
| 6 | 2.06% | `java.lang.ThreadLocal.get() line: 171` |
| 7 | 1.45% | `java.util.ArrayList.get(int) line: 429` |
| 8 | 1.12% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 9 | 0.59% | `java.util.ArrayList.grow(int) line: 238` |
| 10 | 0.58% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 11 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 14 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7997` |
| 18 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12086` |
| 21 | 0.23% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7903` |
| 22 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7982` |
| 23 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.21% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.18% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.17% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8073` |
| 29 | 0.16% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


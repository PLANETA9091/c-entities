# JVM hotspots (JFR ExecutionSample)

Samples: 26470

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.92% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1894` |
| 2 | 6.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3749` |
| 3 | 3.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3748` |
| 4 | 2.43% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 2.11% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3469` |
| 6 | 1.93% | `java.util.ArrayList.add(Object) line: 497` |
| 7 | 1.81% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3816` |
| 9 | 1.60% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1895` |
| 11 | 1.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.01% | `java.util.ArrayList.elementData(int) line: 412` |
| 13 | 0.93% | `java.util.ArrayList.add(Object) line: 496` |
| 14 | 0.87% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 15 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3730` |
| 18 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1893` |
| 20 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.55% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3739` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10487` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 29 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6734` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9879` |
| 32 | 0.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 33 | 0.37% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5232` |
| 34 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 35 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 37 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 38 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10140` |
| 40 | 0.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3731` |
| 42 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 43 | 0.28% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 44 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3732` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10318` |
| 47 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 49 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11110` |
| 51 | 0.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.23% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 53 | 0.23% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10565` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6671` |
| 57 | 0.21% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.21% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3763` |
| 60 | 0.19% | `java.util.EnumMap.get(Object) line: 248` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.91% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 14.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3469` |
| 3 | 5.10% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3817` |
| 5 | 2.11% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1886` |
| 6 | 1.60% | `java.lang.ThreadLocal.get() line: 171` |
| 7 | 1.10% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.01% | `java.util.ArrayList.get(int) line: 429` |
| 11 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10580` |
| 14 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 16 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10565` |
| 18 | 0.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 19 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.56% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 22 | 0.54% | `java.util.ArrayList.grow(int) line: 238` |
| 23 | 0.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.45% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10486` |
| 27 | 0.39% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 29 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |

GC pauses: 0, total 0 ms


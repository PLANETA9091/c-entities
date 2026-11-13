# JVM hotspots (JFR ExecutionSample)

Samples: 85202

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.14% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1377` |
| 2 | 7.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2882` |
| 3 | 4.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2881` |
| 4 | 4.65% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 4.17% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 6 | 3.79% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2949` |
| 7 | 3.63% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 9 | 1.98% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 11 | 1.59% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 12 | 1.51% | `java.util.ArrayList.add(Object) line: 496` |
| 13 | 1.50% | `java.util.ArrayList.elementData(int) line: 412` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2864` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2896` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2880` |
| 17 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 18 | 0.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 19 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2914` |
| 20 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11849` |
| 24 | 0.23% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.21% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2863` |
| 27 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 28 | 0.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 30 | 0.15% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7722` |
| 31 | 0.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7891` |
| 33 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.14% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7544` |
| 35 | 0.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2872` |
| 36 | 0.13% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 37 | 0.12% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 39 | 0.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.12% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.12% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 42 | 0.12% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 43 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5205` |
| 44 | 0.11% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 46 | 0.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.10% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.10% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.10% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5514` |
| 50 | 0.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 51 | 0.09% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5594` |
| 52 | 0.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2777` |
| 53 | 0.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 54 | 0.09% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 55 | 0.09% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 56 | 0.09% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |
| 57 | 0.08% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.08% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.08% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.08% | `java.util.EnumMap.get(Object) line: 248` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.46% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 20.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2645` |
| 3 | 10.81% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 5.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2950` |
| 5 | 2.63% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1369` |
| 6 | 1.59% | `java.lang.ThreadLocal.get() line: 171` |
| 7 | 1.50% | `java.util.ArrayList.get(int) line: 429` |
| 8 | 0.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 0.46% | `java.util.ArrayList.grow(int) line: 238` |
| 10 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 0.32% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 12 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 16 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7984` |
| 20 | 0.22% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 21 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7890` |
| 24 | 0.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7969` |
| 25 | 0.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8060` |
| 26 | 0.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.16% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.16% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.16% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.13% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms


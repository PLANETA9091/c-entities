# JVM hotspots (JFR ExecutionSample)

Samples: 23218

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 2.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 3 | 2.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3075` |
| 4 | 1.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3074` |
| 5 | 1.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3056` |
| 7 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 8 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 0.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8304` |
| 14 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.71% | `java.util.ArrayList.add(Object) line: 497` |
| 16 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 17 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 21 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.61% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8135` |
| 25 | 0.52% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7957` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3057` |
| 29 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 31 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8927` |
| 33 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.37% | `java.util.ArrayList.elementData(int) line: 412` |
| 35 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 36 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 37 | 0.36% | `java.util.ArrayList.add(Object) line: 496` |
| 38 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5459` |
| 40 | 0.35% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 41 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.33% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12852` |
| 45 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 47 | 0.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 48 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5522` |
| 51 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 52 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 54 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.28% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7984` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.27% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.96% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2838` |
| 3 | 2.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 4 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 7 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 9 | 1.09% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 10 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 11 | 1.03% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 12 | 1.01% | `java.util.ArrayList.add(Object) line: 497` |
| 13 | 0.95% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8397` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8382` |
| 19 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8303` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8473` |
| 21 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.65% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.55% | `java.lang.ThreadLocal.get() line: 171` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


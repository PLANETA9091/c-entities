# JVM hotspots (JFR ExecutionSample)

Samples: 16221

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.89% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1454` |
| 2 | 2.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 3 | 2.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 4 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 7 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.91% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 11 | 0.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 12 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 13 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.73% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9268` |
| 17 | 0.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.62% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 19 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6078` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4669` |
| 22 | 0.56% | `java.util.ArrayList.add(Object) line: 497` |
| 23 | 0.55% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3261` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6015` |
| 26 | 0.51% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8921` |
| 28 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 29 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9891` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9099` |
| 32 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.36% | `java.util.ArrayList.elementData(int) line: 412` |
| 37 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 38 | 0.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 39 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 42 | 0.33% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1455` |
| 43 | 0.33% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.28% | `java.lang.String.hashCode() line: 2433` |
| 47 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 48 | 0.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 49 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9346` |
| 50 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.27% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6324` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 55 | 0.25% | `java.util.EnumMap.get(Object) line: 248` |
| 56 | 0.25% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12991` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 58 | 0.24% | `java.util.ArrayList.add(Object) line: 496` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.24% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.50% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 3 | 1.89% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 1.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 9 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.09% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9346` |
| 13 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9361` |
| 16 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1446` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3262` |
| 20 | 0.78% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.76% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.76% | `java.util.HashMap.get(Object) line: 565` |
| 23 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9267` |
| 25 | 0.58% | `java.lang.ThreadLocal.get() line: 171` |
| 26 | 0.57% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.54% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9437` |
| 29 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms


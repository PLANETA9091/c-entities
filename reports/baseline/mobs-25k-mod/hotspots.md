# JVM hotspots (JFR ExecutionSample)

Samples: 21999

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.96% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1786` |
| 2 | 6.01% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3572` |
| 4 | 2.01% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 1.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3573` |
| 6 | 1.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3293` |
| 8 | 1.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.25% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1787` |
| 10 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.99% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.97% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 15 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9772` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9694` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3334` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9525` |
| 22 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9347` |
| 24 | 0.57% | `java.util.ArrayList.add(Object) line: 497` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6402` |
| 26 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6465` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3555` |
| 29 | 0.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 30 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 33 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 34 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 35 | 0.39% | `java.util.ArrayList.add(Object) line: 496` |
| 36 | 0.39% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 37 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9374` |
| 39 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 41 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10317` |
| 42 | 0.35% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 43 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3640` |
| 44 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3587` |
| 46 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3563` |
| 47 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3554` |
| 49 | 0.28% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 50 | 0.28% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 51 | 0.27% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 54 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.24% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.24% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6711` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.26% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 9.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3293` |
| 3 | 6.01% | `java.util.ArrayList.get(int) line: 429` |
| 4 | 2.68% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9787` |
| 8 | 1.35% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.29% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1778` |
| 11 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9772` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3641` |
| 19 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9863` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9693` |
| 22 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.69% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9509` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.47% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.38% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6712` |

GC pauses: 0, total 0 ms


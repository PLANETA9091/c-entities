# JVM hotspots (JFR ExecutionSample)

Samples: 28737

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.83% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1844` |
| 2 | 3.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3698` |
| 3 | 3.51% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 3.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3699` |
| 5 | 1.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3419` |
| 6 | 1.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1845` |
| 7 | 1.03% | `java.util.ArrayList.elementData(int) line: 412` |
| 8 | 0.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.75% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1843` |
| 10 | 0.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3681` |
| 11 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3460` |
| 12 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.69% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 14 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3697` |
| 16 | 0.61% | `java.util.ArrayList.add(Object) line: 497` |
| 17 | 0.58% | `java.util.ArrayList.grow() line: 245` |
| 18 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3766` |
| 20 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3731` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1846` |
| 23 | 0.43% | `java.util.Objects.checkIndex(int, int) line: 365` |
| 24 | 0.41% | `java.util.ArrayList.add(Object) line: 496` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 27 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 30 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10061` |
| 33 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3682` |
| 35 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 36 | 0.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10139` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9714` |
| 39 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 40 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6684` |
| 42 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 44 | 0.22% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 45 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 46 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10684` |
| 48 | 0.21% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9892` |
| 51 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 53 | 0.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 54 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.18% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.18% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 59 | 0.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 60 | 0.17% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.50% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 11.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3419` |
| 3 | 4.56% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 1.64% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1836` |
| 5 | 1.13% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.03% | `java.util.ArrayList.get(int) line: 429` |
| 7 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3767` |
| 8 | 0.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10154` |
| 11 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.58% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 15 | 0.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 16 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 18 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 0.43% | `java.util.ArrayList.get(int) line: 428` |
| 21 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10139` |
| 22 | 0.42% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10060` |
| 25 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.31% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 30 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms


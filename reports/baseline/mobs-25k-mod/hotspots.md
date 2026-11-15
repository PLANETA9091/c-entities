# JVM hotspots (JFR ExecutionSample)

Samples: 34055

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 43.16% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 3.98% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1464` |
| 3 | 1.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 4 | 1.29% | `java.util.ArrayList.elementData(int) line: 412` |
| 5 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3203` |
| 6 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 0.73% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 8 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2967` |
| 9 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13654` |
| 11 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3204` |
| 14 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9278` |
| 15 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.40% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 20 | 0.38% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1465` |
| 21 | 0.38% | `java.util.ArrayList.add(Object) line: 497` |
| 22 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 23 | 0.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3191` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9109` |
| 26 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 27 | 0.33% | `java.util.ArrayList.add(Object) line: 496` |
| 28 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 30 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8931` |
| 32 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6025` |
| 33 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.27% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 38 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.23% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3185` |
| 41 | 0.22% | `java.util.HashMap.getNode(Object) line: 586` |
| 42 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3218` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9901` |
| 45 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 46 | 0.20% | `java.util.Collections$UnmodifiableCollection$1.hasNext() line: 1079` |
| 47 | 0.20% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.20% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6000` |
| 50 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 51 | 0.19% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8958` |
| 53 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14307` |
| 55 | 0.18% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6088` |
| 57 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.17% | `java.lang.ThreadLocal.get() line: 171` |
| 59 | 0.17% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 60 | 0.16% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 43.16% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 4.46% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 3.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2967` |
| 4 | 1.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 5 | 1.44% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 1.29% | `java.util.ArrayList.get(int) line: 429` |
| 7 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 0.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3272` |
| 12 | 0.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13932` |
| 13 | 0.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 0.62% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1456` |
| 15 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.60% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 0.53% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9371` |
| 19 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9356` |
| 23 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9277` |
| 25 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3006` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9447` |
| 28 | 0.36% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.34% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 30 | 0.34% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6000` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 22476

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.60% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 2.60% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1454` |
| 3 | 1.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 5 | 1.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 8 | 0.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.87% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 10 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9242` |
| 12 | 0.74% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9073` |
| 13 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1455` |
| 16 | 0.73% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 17 | 0.70% | `java.util.ArrayList.elementData(int) line: 412` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9320` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8895` |
| 21 | 0.64% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 22 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 25 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6005` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 29 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6068` |
| 32 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8922` |
| 34 | 0.40% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 35 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 37 | 0.33% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9865` |
| 38 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6397` |
| 43 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.29% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 45 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.26% | `java.util.ArrayList.grow() line: 245` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14271` |
| 51 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 52 | 0.24% | `java.lang.String.hashCode() line: 2430` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3184` |
| 54 | 0.24% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.22% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6394` |
| 59 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3089` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 3.43% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 4 | 1.92% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9335` |
| 9 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9320` |
| 15 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9411` |
| 16 | 0.90% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.86% | `java.util.ArrayList.add(Object) line: 497` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9241` |
| 19 | 0.74% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.70% | `java.util.ArrayList.get(int) line: 429` |
| 22 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9057` |
| 25 | 0.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 26 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 27 | 0.50% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.49% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1446` |
| 30 | 0.45% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


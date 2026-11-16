# JVM hotspots (JFR ExecutionSample)

Samples: 21983

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.48% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1581` |
| 2 | 3.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3368` |
| 3 | 3.09% | `java.util.ArrayList.elementData(int) line: 412` |
| 4 | 3.03% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 2.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3367` |
| 6 | 1.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3435` |
| 7 | 1.46% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |
| 8 | 1.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.08% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1582` |
| 11 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 12 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.98% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 0.97% | `java.util.ArrayList.add(Object) line: 496` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.91% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 17 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3350` |
| 22 | 0.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 23 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.61% | `java.util.ArrayList.add(Object) line: 497` |
| 25 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9489` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3382` |
| 29 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9142` |
| 31 | 0.45% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9320` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6197` |
| 34 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 38 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6260` |
| 43 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 44 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 45 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14518` |
| 47 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3400` |
| 48 | 0.30% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 49 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 51 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.26% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10112` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6614` |
| 55 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 57 | 0.25% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9169` |
| 58 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 10.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3088` |
| 3 | 5.59% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 3.09% | `java.util.ArrayList.get(int) line: 429` |
| 5 | 2.25% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3436` |
| 7 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 11 | 1.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 0.96% | `java.util.ArrayList.grow(int) line: 238` |
| 14 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9582` |
| 17 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9567` |
| 20 | 0.81% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9488` |
| 24 | 0.62% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9658` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |

GC pauses: 0, total 0 ms


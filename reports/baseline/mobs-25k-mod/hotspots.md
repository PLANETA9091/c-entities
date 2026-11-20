# JVM hotspots (JFR ExecutionSample)

Samples: 47988

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 56.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3910` |
| 2 | 7.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3982` |
| 3 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3924` |
| 4 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3900` |
| 5 | 1.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 6 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3909` |
| 7 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15343` |
| 11 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3892` |
| 12 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 18 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10754` |
| 21 | 0.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 22 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3891` |
| 23 | 0.22% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10585` |
| 24 | 0.22% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10407` |
| 25 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.20% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.20% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.18% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 31 | 0.18% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 32 | 0.17% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.16% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.16% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16122` |
| 35 | 0.15% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 38 | 0.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 39 | 0.13% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7296` |
| 40 | 0.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.13% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11377` |
| 42 | 0.13% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.13% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 44 | 0.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3908` |
| 45 | 0.12% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.12% | `java.lang.Enum.ordinal() line: 128` |
| 47 | 0.12% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6904` |
| 48 | 0.12% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 49 | 0.12% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 50 | 0.12% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 51 | 0.12% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.12% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.11% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 54 | 0.11% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 55 | 0.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.11% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 57 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6967` |
| 58 | 0.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 59 | 0.11% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10434` |
| 60 | 0.11% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2039` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 68.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2039` |
| 2 | 1.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 3 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 4 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 0.45% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 0.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15623` |
| 10 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10847` |
| 11 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.38% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10832` |
| 15 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 16 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 0.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10753` |
| 18 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 21 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.29% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10923` |
| 24 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.21% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.21% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.20% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms


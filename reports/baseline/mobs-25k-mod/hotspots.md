# JVM hotspots (JFR ExecutionSample)

Samples: 22297

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.22% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1581` |
| 2 | 2.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3320` |
| 3 | 2.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3321` |
| 4 | 2.00% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 5 | 0.99% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 6 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13818` |
| 10 | 0.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.74% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 12 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 13 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 16 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.63% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1582` |
| 18 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3388` |
| 21 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3084` |
| 23 | 0.53% | `java.util.ArrayList.add(Object) line: 497` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9442` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9095` |
| 26 | 0.47% | `java.util.ArrayList.elementData(int) line: 412` |
| 27 | 0.46% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 28 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 29 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 30 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.40% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 36 | 0.40% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9273` |
| 37 | 0.37% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 38 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 39 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3303` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6213` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6150` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6459` |
| 43 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10065` |
| 45 | 0.30% | `java.util.ArrayList.add(Object) line: 496` |
| 46 | 0.30% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13165` |
| 47 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.28% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 50 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14471` |
| 51 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.26% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 53 | 0.26% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkStatus() line: 1112` |
| 57 | 0.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 58 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9122` |
| 60 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.86% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 7.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3084` |
| 3 | 2.13% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.07% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 2.00% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 6 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 7 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14096` |
| 9 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 10 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.89% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3389` |
| 16 | 0.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9535` |
| 19 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9441` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9520` |
| 24 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1573` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.52% | `java.util.HashMap.get(Object) line: 565` |
| 29 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9611` |
| 30 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


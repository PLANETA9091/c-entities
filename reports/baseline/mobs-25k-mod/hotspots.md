# JVM hotspots (JFR ExecutionSample)

Samples: 59158

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 59.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3938` |
| 2 | 6.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4010` |
| 3 | 2.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3937` |
| 4 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3920` |
| 5 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 6 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 7 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3970` |
| 11 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15535` |
| 12 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3919` |
| 16 | 0.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3928` |
| 20 | 0.25% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.24% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.23% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10932` |
| 24 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.22% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10585` |
| 26 | 0.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 27 | 0.20% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 28 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.19% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10763` |
| 30 | 0.18% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 31 | 0.17% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2048` |
| 32 | 0.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.14% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7055` |
| 35 | 0.14% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16314` |
| 36 | 0.14% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.14% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 38 | 0.14% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 40 | 0.13% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.12% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 42 | 0.12% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10612` |
| 43 | 0.11% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6992` |
| 45 | 0.11% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 46 | 0.11% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6967` |
| 47 | 0.11% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 48 | 0.10% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 50 | 0.10% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.10% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.10% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 53 | 0.10% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 54 | 0.10% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 55 | 0.10% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.10% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11555` |
| 58 | 0.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 59 | 0.09% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.09% | `net.minecraft.world.entity.ai.attributes.AttributeSupplier.getAttributeInstance(Holder) line: 18` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 72.45% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2048` |
| 2 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 3 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 4 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15815` |
| 10 | 0.37% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 11 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11025` |
| 12 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11010` |
| 13 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10931` |
| 16 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.28% | `java.util.HashMap.get(Object) line: 565` |
| 19 | 0.28% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.27% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.25% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11101` |
| 25 | 0.22% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.18% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10747` |
| 28 | 0.18% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 29 | 0.18% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6967` |
| 30 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms


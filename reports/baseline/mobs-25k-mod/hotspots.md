# JVM hotspots (JFR ExecutionSample)

Samples: 21296

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.95% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 4.00% | `java.util.ArrayList.elementData(int) line: 412` |
| 3 | 2.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3118` |
| 5 | 2.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3119` |
| 6 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 9 | 0.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.89% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 12 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.77% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 16 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3186` |
| 18 | 0.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 20 | 0.63% | `java.util.ArrayList.add(Object) line: 497` |
| 21 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8352` |
| 23 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2923` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 27 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8183` |
| 29 | 0.48% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3014` |
| 31 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 32 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.42% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8005` |
| 35 | 0.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3101` |
| 36 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 40 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.33% | `java.util.ArrayList.grow() line: 245` |
| 42 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.30% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8975` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5503` |
| 46 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 47 | 0.29% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 48 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 49 | 0.28% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8032` |
| 50 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 51 | 0.27% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 52 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.26% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 56 | 0.26% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12900` |
| 57 | 0.25% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5478` |
| 58 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 60 | 0.24% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.56% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 8.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 3 | 4.00% | `java.util.ArrayList.get(int) line: 429` |
| 4 | 2.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 5 | 2.07% | `java.util.ArrayList.add(Object) line: 497` |
| 6 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 1.02% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 12 | 0.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8445` |
| 14 | 0.88% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3187` |
| 16 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8430` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8521` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8351` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5478` |

GC pauses: 0, total 0 ms


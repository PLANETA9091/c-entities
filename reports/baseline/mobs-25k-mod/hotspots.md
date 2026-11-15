# JVM hotspots (JFR ExecutionSample)

Samples: 22826

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.33% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 3.79% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1524` |
| 3 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 0.97% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 0.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3263` |
| 10 | 0.85% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 11 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.79% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1525` |
| 13 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9338` |
| 14 | 0.76% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9169` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9416` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13714` |
| 18 | 0.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8991` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6085` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3027` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9018` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 25 | 0.46% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 26 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 28 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6148` |
| 29 | 0.44% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 30 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 31 | 0.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 32 | 0.38% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 34 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9961` |
| 36 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.32% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 38 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3246` |
| 40 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 41 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.29% | `java.util.ArrayList.grow() line: 245` |
| 45 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.27% | `java.util.ArrayList.add(Object) line: 496` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3331` |
| 48 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 49 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 50 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 52 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8794` |
| 55 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.24% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3159` |
| 58 | 0.24% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 6060` |
| 59 | 0.24% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6394` |
| 60 | 0.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 4.70% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3027` |
| 4 | 1.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 1.69% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9431` |
| 8 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9416` |
| 10 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.07% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 0.90% | `java.util.ArrayList.add(Object) line: 497` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9507` |
| 17 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9337` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13992` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9153` |
| 21 | 0.67% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1516` |
| 26 | 0.53% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 28 | 0.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3332` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6395` |
| 30 | 0.44% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms


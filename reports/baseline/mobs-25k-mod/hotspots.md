# JVM hotspots (JFR ExecutionSample)

Samples: 48855

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 55.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 2 | 5.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4024` |
| 3 | 2.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3966` |
| 4 | 1.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3942` |
| 5 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3951` |
| 6 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3934` |
| 8 | 0.66% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 12 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 13 | 0.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11206` |
| 17 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3950` |
| 18 | 0.34% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 19 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11284` |
| 22 | 0.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3804` |
| 23 | 0.28% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11037` |
| 24 | 0.27% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 25 | 0.27% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 26 | 0.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 27 | 0.26% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 28 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.23% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10859` |
| 30 | 0.21% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 32 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 33 | 0.20% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.20% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10886` |
| 35 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.19% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.19% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.18% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.18% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.17% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.17% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.16% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 45 | 0.16% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7146` |
| 46 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 47 | 0.16% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 48 | 0.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 49 | 0.15% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11829` |
| 50 | 0.15% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 51 | 0.15% | `java.util.HashMap.getNode(Object) line: 586` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7209` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7538` |
| 55 | 0.14% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 56 | 0.14% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 57 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.14% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16658` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 66.78% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2062` |
| 2 | 0.92% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11299` |
| 6 | 0.65% | `java.util.ArrayList.grow(int) line: 238` |
| 7 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 10 | 0.42% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 14 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11375` |
| 16 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11205` |
| 17 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11284` |
| 19 | 0.34% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 20 | 0.34% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.28% | `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal.findTarget() line: 66` |
| 24 | 0.27% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 25 | 0.26% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16126` |
| 27 | 0.26% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11021` |
| 29 | 0.24% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


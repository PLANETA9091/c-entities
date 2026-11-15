# JVM hotspots (JFR ExecutionSample)

Samples: 18652

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1431` |
| 2 | 3.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3170` |
| 3 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3171` |
| 6 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.26% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.24% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 9 | 1.21% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.10% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 12 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2934` |
| 14 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.95% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3066` |
| 17 | 0.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9125` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9203` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.70% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 22 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13460` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8956` |
| 26 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5985` |
| 30 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.48% | `java.util.ArrayList.elementData(int) line: 412` |
| 32 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9748` |
| 34 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5922` |
| 35 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8778` |
| 39 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 41 | 0.39% | `java.util.ArrayList.add(Object) line: 497` |
| 42 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3238` |
| 46 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1432` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14077` |
| 49 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 50 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.32% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 52 | 0.31% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 49` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.30% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1430` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5897` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6314` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.99% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 7.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2934` |
| 3 | 1.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.98% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9218` |
| 6 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.54% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.26% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 13 | 1.10% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1423` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9203` |
| 16 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13702` |
| 22 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9124` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9294` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3239` |
| 28 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


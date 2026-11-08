# JVM hotspots (JFR ExecutionSample)

Samples: 17112

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2878` |
| 2 | 2.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 1.52% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 5 | 1.50% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 6 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2744` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7640` |
| 12 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7718` |
| 14 | 0.90% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 16 | 0.86% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10854` |
| 18 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7293` |
| 22 | 0.81% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7471` |
| 23 | 0.79% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 24 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 26 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 27 | 0.60% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8263` |
| 29 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.57% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.57% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7320` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 33 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.52% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1313` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5079` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5471` |
| 38 | 0.49% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5142` |
| 43 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.40% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 47 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.34% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.33% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11443` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5054` |
| 56 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |
| 59 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5388` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2612` |
| 2 | 2.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 2.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.30% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 1.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7733` |
| 6 | 1.52% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 7 | 1.50% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 8 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2872` |
| 10 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |
| 14 | 1.05% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7639` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7809` |
| 17 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7718` |
| 19 | 0.92% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.84% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7455` |
| 30 | 0.74% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


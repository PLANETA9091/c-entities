# JVM hotspots (JFR ExecutionSample)

Samples: 74942

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 69.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 2 | 4.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4024` |
| 3 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3966` |
| 4 | 1.13% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 5 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3950` |
| 6 | 0.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3934` |
| 7 | 0.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3942` |
| 8 | 0.64% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3984` |
| 9 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4025` |
| 10 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3935` |
| 11 | 0.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.31% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2062` |
| 14 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.23% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16008` |
| 19 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3951` |
| 20 | 0.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11368` |
| 22 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.14% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 26 | 0.14% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 27 | 0.13% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.12% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11021` |
| 31 | 0.11% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.11% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 34 | 0.11% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11446` |
| 35 | 0.11% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 36 | 0.11% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 37 | 0.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.10% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 39 | 0.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3804` |
| 40 | 0.10% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.09% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 42 | 0.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 43 | 0.08% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7608` |
| 44 | 0.08% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7194` |
| 45 | 0.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3933` |
| 46 | 0.08% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 47 | 0.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 48 | 0.08% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 49 | 0.08% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.08% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11199` |
| 51 | 0.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10760` |
| 52 | 0.07% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7282` |
| 53 | 0.07% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 54 | 0.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.07% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.07% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 58 | 0.07% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 59 | 0.07% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7219` |
| 60 | 0.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 80.03% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2062` |
| 2 | 1.12% | `java.util.ArrayList.grow(int) line: 238` |
| 3 | 0.49% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 0.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 0.32% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 7 | 0.30% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 8 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 9 | 0.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11461` |
| 10 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 0.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 13 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.21% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 16 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16288` |
| 17 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 0.17% | `java.util.HashMap.get(Object) line: 565` |
| 19 | 0.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 0.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11367` |
| 21 | 0.16% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.15% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11446` |
| 24 | 0.14% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 25 | 0.14% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.14% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 27 | 0.13% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.11% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms


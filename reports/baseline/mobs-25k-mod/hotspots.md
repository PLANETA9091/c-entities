# JVM hotspots (JFR ExecutionSample)

Samples: 60098

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 64.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 2 | 6.16% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4024` |
| 3 | 1.29% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3966` |
| 4 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3950` |
| 5 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3933` |
| 6 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3934` |
| 7 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3942` |
| 8 | 0.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3984` |
| 9 | 0.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3935` |
| 10 | 0.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 15 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.20% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2062` |
| 19 | 0.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11352` |
| 20 | 0.20% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 21 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.18% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.17% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11005` |
| 24 | 0.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.17% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 26 | 0.17% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 27 | 0.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 28 | 0.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.15% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.15% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3951` |
| 33 | 0.14% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.13% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11430` |
| 35 | 0.12% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.12% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 37 | 0.12% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 38 | 0.12% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 39 | 0.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 40 | 0.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 41 | 0.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3804` |
| 42 | 0.11% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11183` |
| 43 | 0.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.11% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7283` |
| 45 | 0.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 47 | 0.10% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.09% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 49 | 0.09% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11975` |
| 50 | 0.09% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7529` |
| 51 | 0.09% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.09% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.08% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7220` |
| 55 | 0.08% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.08% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7195` |
| 57 | 0.08% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 58 | 0.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.08% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 11032` |
| 60 | 0.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 76.75% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2062` |
| 2 | 0.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 0.39% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11445` |
| 7 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 8 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.26% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16272` |
| 12 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 13 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 0.23% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11351` |
| 15 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.22% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11430` |
| 17 | 0.22% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 18 | 0.21% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 19 | 0.20% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.20% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 21 | 0.19% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 22 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.17% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 25 | 0.17% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 26 | 0.16% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.16% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.15% | `java.util.HashMap.get(Object) line: 565` |
| 29 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


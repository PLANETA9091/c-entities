# JVM hotspots (JFR ExecutionSample)

Samples: 58548

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 59.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4015` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4087` |
| 3 | 2.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4014` |
| 4 | 1.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3997` |
| 5 | 1.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4029` |
| 6 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 7 | 0.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4047` |
| 9 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3996` |
| 13 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.27% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 18 | 0.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11480` |
| 19 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4005` |
| 21 | 0.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 23 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.19% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11311` |
| 25 | 0.19% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11133` |
| 26 | 0.18% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 27 | 0.18% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2117` |
| 28 | 0.17% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7321` |
| 29 | 0.17% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.16% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.16% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.15% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 33 | 0.15% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 34 | 0.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 35 | 0.14% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.12% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7384` |
| 37 | 0.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 39 | 0.12% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 12103` |
| 40 | 0.11% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 7296` |
| 41 | 0.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.11% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.11% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.11% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.11% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.11% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.10% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.10% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 17319` |
| 49 | 0.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4013` |
| 50 | 0.10% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 51 | 0.10% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 52 | 0.10% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.10% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 934` |
| 54 | 0.09% | `java.lang.ThreadLocal.get() line: 171` |
| 55 | 0.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.09% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 57 | 0.09% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 58 | 0.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.09% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 11160` |
| 60 | 0.09% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 72.77% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2117` |
| 2 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 3 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 5 | 0.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.39% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 0.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 10 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11573` |
| 12 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.29% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16727` |
| 16 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 17 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11558` |
| 18 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.26% | `java.util.HashMap.get(Object) line: 565` |
| 20 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11479` |
| 21 | 0.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11649` |
| 22 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.21% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 25 | 0.20% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.19% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 27 | 0.18% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos) line: 863` |
| 28 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.17% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


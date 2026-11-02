# JVM hotspots (JFR ExecutionSample)

Samples: 11859

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9819` |
| 2 | 2.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9483` |
| 9 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.11% | `java.lang.ref.Reference.get() line: 361` |
| 13 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.78% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.71% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4464` |
| 23 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.55% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 25 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 26 | 0.54% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7246` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4311` |
| 29 | 0.52% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 30 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 33 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4641` |
| 35 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9344` |
| 37 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 38 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 39 | 0.46% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 40 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9764` |
| 43 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 46 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 47 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 55 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 56 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4286` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 1.88% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.18% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 1.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4484` |
| 15 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 16 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9427` |
| 22 | 0.84% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.77% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4642` |
| 26 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4640` |
| 30 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms


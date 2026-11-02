# JVM hotspots (JFR ExecutionSample)

Samples: 14405

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9543` |
| 4 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 6 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 7 | 1.49% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 8 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 10 | 1.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.03% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 18 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 23 | 0.71% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.69% | `java.lang.ref.Reference.get() line: 361` |
| 25 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 26 | 0.64% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4366` |
| 29 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 32 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4303` |
| 34 | 0.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 35 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7196` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4456` |
| 38 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4633` |
| 44 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 45 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 46 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 521` |
| 48 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 51 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 56 | 0.30% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 57 | 0.30% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 60 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9220` |
| 2 | 2.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 1.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 6 | 1.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 9 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.17% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 1.06% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.93% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 25 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5962` |
| 26 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 28 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.72% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


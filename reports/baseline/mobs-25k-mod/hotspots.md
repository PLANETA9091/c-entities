# JVM hotspots (JFR ExecutionSample)

Samples: 12527

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 1.93% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10235` |
| 3 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.49% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9633` |
| 10 | 1.19% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.06% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6854` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |
| 16 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6520` |
| 18 | 0.74% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6698` |
| 22 | 0.64% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6347` |
| 26 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.51% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7464` |
| 29 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 31 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.44% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6547` |
| 34 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 35 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4355` |
| 43 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 45 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 46 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.35% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 48 | 0.34% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 49 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 51 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.33% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 53 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 56 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 3 | 1.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6937` |
| 8 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6922` |
| 10 | 1.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 12 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9781` |
| 15 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6853` |
| 17 | 1.17% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 1.06% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 19 | 1.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.85% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7010` |
| 24 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.75% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6682` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |

GC pauses: 0, total 0 ms


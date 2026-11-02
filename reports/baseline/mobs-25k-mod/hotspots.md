# JVM hotspots (JFR ExecutionSample)

Samples: 11705

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.12% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9898` |
| 4 | 2.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9562` |
| 10 | 1.29% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.98% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.91% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 16 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.86% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4398` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9965` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4335` |
| 24 | 0.58% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4488` |
| 26 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7284` |
| 28 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 31 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 35 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 37 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.44% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4665` |
| 42 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 45 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 50 | 0.36% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 51 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 56 | 0.32% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 60 | 0.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 1.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 5 | 1.60% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.18% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9494` |
| 17 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 0.91% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 21 | 0.89% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4508` |
| 25 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 29 | 0.73% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 30 | 0.73% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4666` |

GC pauses: 0, total 0 ms


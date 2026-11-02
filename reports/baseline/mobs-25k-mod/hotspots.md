# JVM hotspots (JFR ExecutionSample)

Samples: 15274

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9334` |
| 5 | 1.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9193` |
| 8 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.05% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.91% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.85% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 18 | 0.84% | `java.lang.ref.Reference.get() line: 361` |
| 19 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9107` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7182` |
| 22 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4355` |
| 25 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 26 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5998` |
| 28 | 0.53% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6152` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4292` |
| 31 | 0.51% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4445` |
| 32 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.48% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.48% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 38 | 0.46% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 39 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4622` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 42 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9279` |
| 44 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 52 | 0.31% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 54 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 829` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 58 | 0.28% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 59 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 2 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 5 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9206` |
| 8 | 1.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9181` |
| 9 | 1.55% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 15 | 1.04% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 16 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 20 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.94% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 22 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.84% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4465` |
| 25 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9189` |
| 28 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 13768

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9149` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 7 | 1.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.00% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 20 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.66% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7061` |
| 23 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4171` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5877` |
| 26 | 0.52% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 28 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4234` |
| 32 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.42% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4324` |
| 35 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 36 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 40 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 41 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4501` |
| 44 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 45 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4146` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9094` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9021` |
| 4 | 1.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.69% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.58% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8996` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 12 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 18 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5827` |
| 20 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 24 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 25 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 26 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.79% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.79% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 29 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 14128

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 6 | 1.71% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 9 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 16 | 1.03% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 21 | 0.62% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 24 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 25 | 0.58% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4214` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7041` |
| 29 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 31 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 34 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 38 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 40 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 42 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4304` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4151` |
| 46 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4126` |
| 52 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9074` |
| 56 | 0.30% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 57 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 59 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 4 | 2.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 5 | 1.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.56% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 12 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 15 | 1.10% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.01% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 22 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 23 | 0.88% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.83% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 25 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 26 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.74% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.68% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 14609

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 7 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9541` |
| 10 | 1.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 13 | 1.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 16 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 17 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4387` |
| 21 | 0.66% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 22 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7273` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4324` |
| 26 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.50% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 28 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4477` |
| 30 | 0.46% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 31 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 32 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.44% | `java.lang.Enum.ordinal() line: 128` |
| 34 | 0.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 35 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 36 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 37 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4654` |
| 40 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.40% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 42 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.37% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9822` |
| 48 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.34% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 50 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 51 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 53 | 0.31% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 54 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 55 | 0.31% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 56 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.29% | `net.minecraft.world.entity.Mob.serverAiStep() line: 829` |
| 60 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.16% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 2.15% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 4 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.87% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 1.26% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 14 | 1.21% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9473` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7269` |
| 22 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.88% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4497` |
| 28 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.74% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.73% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms


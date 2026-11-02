# JVM hotspots (JFR ExecutionSample)

Samples: 15686

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9345` |
| 5 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 13 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 17 | 0.71% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 19 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.57% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 24 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7193` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4363` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4453` |
| 29 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 32 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 34 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4300` |
| 37 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 44 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 45 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 47 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 48 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4630` |
| 50 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4275` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4707` |
| 56 | 0.28% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 58 | 0.27% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 59 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 60 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.56% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 2 | 2.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9192` |
| 10 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.13% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5959` |
| 20 | 0.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 22 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.85% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9200` |
| 27 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4629` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4473` |
| 30 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms


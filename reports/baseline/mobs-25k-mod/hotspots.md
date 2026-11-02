# JVM hotspots (JFR ExecutionSample)

Samples: 14714

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9257` |
| 6 | 1.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9116` |
| 9 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 13 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.01% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 0.89% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5985` |
| 21 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.58% | `java.lang.ref.Reference.get() line: 361` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6139` |
| 26 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4342` |
| 28 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4279` |
| 30 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.43% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 32 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.43% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4432` |
| 34 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9063` |
| 38 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 39 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4609` |
| 41 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7169` |
| 44 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 45 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 46 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 48 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 52 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.31% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 55 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 57 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.29% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 59 | 0.28% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 2 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 1.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9129` |
| 7 | 1.72% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 1.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9104` |
| 11 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.28% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.19% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 19 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5935` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.75% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 27 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 28 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |

GC pauses: 0, total 0 ms


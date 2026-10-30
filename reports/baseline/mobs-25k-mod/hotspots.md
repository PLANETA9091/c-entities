# JVM hotspots (JFR ExecutionSample)

Samples: 13774

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.93% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 7 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.97% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 20 | 0.76% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3925` |
| 23 | 0.62% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 26 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.52% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 30 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.49% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4233` |
| 32 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 35 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 37 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4078` |
| 41 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 46 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 52 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.33% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 57 | 0.31% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6749` |
| 60 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8709` |
| 3 | 1.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 1.77% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 8 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 12 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 1.08% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 17 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5537` |
| 22 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.76% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4098` |
| 27 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 28 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 30 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms


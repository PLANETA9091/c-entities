# JVM hotspots (JFR ExecutionSample)

Samples: 13906

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8615` |
| 8 | 1.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 9 | 1.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 19 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 20 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3766` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6527` |
| 29 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 31 | 0.48% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3856` |
| 32 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 34 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5365` |
| 37 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3703` |
| 40 | 0.40% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 44 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8682` |
| 47 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 48 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.35% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 50 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 51 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 53 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 55 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 56 | 0.32% | `java.lang.ThreadLocal.get() line: 171` |
| 57 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8487` |
| 3 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.67% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.48% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8462` |
| 8 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 9 | 1.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3112` |
| 17 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.92% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5315` |
| 21 | 0.89% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3876` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4012` |
| 27 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 28 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms


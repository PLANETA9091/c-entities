# JVM hotspots (JFR ExecutionSample)

Samples: 12179

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 7 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.40% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 14 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 17 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 1.00% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3784` |
| 20 | 0.99% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.94% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.85% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 25 | 0.81% | `java.lang.ref.Reference.get() line: 361` |
| 26 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 28 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6545` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 32 | 0.64% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3874` |
| 33 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.56% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3721` |
| 37 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8700` |
| 39 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4029` |
| 47 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 51 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 54 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 58 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 3 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 1.49% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3130` |
| 13 | 1.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 16 | 1.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.12% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 1.08% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 20 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.95% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 25 | 0.89% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.86% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 28 | 0.86% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4030` |
| 29 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.85% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5333` |

GC pauses: 0, total 0 ms


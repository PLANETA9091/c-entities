# JVM hotspots (JFR ExecutionSample)

Samples: 15119

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 6 | 1.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 8 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.01% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.89% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 21 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.57% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 25 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6545` |
| 28 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3874` |
| 30 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3721` |
| 32 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3784` |
| 33 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 36 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 37 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 39 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 47 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4029` |
| 50 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 52 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 53 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 55 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8700` |
| 60 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 1.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 1.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 1.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.76% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 10 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.26% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 12 | 1.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 16 | 1.08% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 18 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3894` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 29 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |

GC pauses: 0, total 0 ms


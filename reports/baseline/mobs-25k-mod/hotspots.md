# JVM hotspots (JFR ExecutionSample)

Samples: 14101

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 2.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8858` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 11 | 1.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 15 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5753` |
| 16 | 1.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 17 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.99% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.98% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 21 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.80% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4009` |
| 27 | 0.72% | `java.lang.ref.Reference.get() line: 361` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6770` |
| 29 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.63% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.59% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3946` |
| 34 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5608` |
| 39 | 0.49% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4099` |
| 46 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8925` |
| 50 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 52 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 53 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8803` |
| 56 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 60 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8730` |
| 4 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.74% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3311` |
| 8 | 1.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.55% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 11 | 1.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8713` |
| 17 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 1.24% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 1.07% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 1.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.98% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 24 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.86% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.82% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4119` |

GC pauses: 0, total 0 ms


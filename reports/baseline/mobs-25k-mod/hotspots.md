# JVM hotspots (JFR ExecutionSample)

Samples: 12571

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.77% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 15 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 1.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 18 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3635` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.99% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.94% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 23 | 0.91% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 24 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3572` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6335` |
| 27 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.73% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 31 | 0.72% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.71% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 33 | 0.68% | `java.lang.ref.Reference.get() line: 361` |
| 34 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 35 | 0.64% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 37 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 39 | 0.59% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.51% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8368` |
| 42 | 0.48% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3880` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3725` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8490` |
| 47 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 54 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.30% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8295` |
| 3 | 2.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 2.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 11 | 1.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 16 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.18% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 1.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 19 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 21 | 1.01% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 22 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 23 | 0.95% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.94% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 25 | 0.91% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3881` |
| 26 | 0.91% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 27 | 0.91% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.89% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6331` |
| 29 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5123` |
| 30 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms


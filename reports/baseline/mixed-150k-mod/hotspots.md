# JVM hotspots (JFR ExecutionSample)

Samples: 34464

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.04% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 3 | 2.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.43% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 7 | 2.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 9 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |
| 11 | 2.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |
| 16 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.19% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 28 | 0.76% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 32 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.65% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 36 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3462` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 39 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 41 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3354` |
| 46 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3745` |
| 48 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 50 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8602` |
| 52 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 56 | 0.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 57 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3918` |
| 59 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3808` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8512` |
| 3 | 2.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.00% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.80% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 11 | 1.78% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8529` |
| 14 | 1.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 1.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4862` |
| 17 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 1.19% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.01% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.94% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.83% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6446` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |
| 29 | 0.75% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 30 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


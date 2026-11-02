# JVM hotspots (JFR ExecutionSample)

Samples: 29826

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6163` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9204` |
| 5 | 2.77% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9345` |
| 7 | 2.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 2.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 2.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9118` |
| 12 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 13 | 1.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 15 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5424` |
| 22 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.95% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.93% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4017` |
| 31 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9163` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4363` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3890` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 44 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 46 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6173` |
| 49 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7193` |
| 56 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.33% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9298` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9200` |
| 5 | 2.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.23% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 2.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9217` |
| 14 | 1.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.42% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9192` |
| 18 | 1.36% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.31% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.26% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 1.08% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 23 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5462` |
| 25 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6009` |
| 26 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6173` |
| 28 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


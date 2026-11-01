# JVM hotspots (JFR ExecutionSample)

Samples: 31788

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 3 | 3.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 5 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 11 | 2.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 12 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 17 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.16% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 1.05% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 26 | 0.82% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 30 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.69% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 32 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.67% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.53% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.50% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8881` |
| 43 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 46 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4146` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.42% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 53 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 55 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6973` |
| 56 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.36% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 58 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9014` |
| 59 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 4.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 6 | 2.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.20% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.48% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8933` |
| 18 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 1.16% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3706` |
| 30 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms


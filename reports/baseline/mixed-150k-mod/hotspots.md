# JVM hotspots (JFR ExecutionSample)

Samples: 28909

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 4 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 5 | 2.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 6 | 2.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 8 | 2.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 2.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 11 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4583` |
| 14 | 1.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 18 | 1.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 19 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.83% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 30 | 0.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 31 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 32 | 0.65% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8949` |
| 40 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 43 | 0.50% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.50% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.48% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 47 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4214` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 53 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4151` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 56 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.35% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9082` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 5 | 2.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 15 | 1.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 16 | 1.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.31% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 1.26% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 1.19% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 22 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 23 | 0.94% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6918` |
| 27 | 0.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 28 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 29 | 0.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 30 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 31285

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 3 | 3.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 4 | 2.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 6 | 2.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 10 | 2.10% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 11 | 1.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 13 | 1.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 14 | 1.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 18 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.91% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 25 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 30 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.55% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 38 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 43 | 0.44% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 44 | 0.44% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 47 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 51 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 53 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 57 | 0.33% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 58 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 59 | 0.32% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 60 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.12% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 5 | 2.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 2.09% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.70% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 13 | 1.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.55% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 17 | 1.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 1.20% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 21 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 1.00% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms


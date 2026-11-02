# JVM hotspots (JFR ExecutionSample)

Samples: 27244

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 3.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 9 | 2.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 17 | 1.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 23 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4041` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9390` |
| 29 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 32 | 0.59% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 34 | 0.57% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 35 | 0.57% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 36 | 0.56% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.53% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 39 | 0.53% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 43 | 0.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 44 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9822` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4387` |
| 48 | 0.40% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3914` |
| 50 | 0.39% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9830` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 54 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 55 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 58 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 59 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9483` |
| 3 | 4.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.84% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 6 | 2.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 2.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.46% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 14 | 1.34% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 1.31% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 16 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 17 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 20 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9473` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3947` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7149` |
| 28 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms


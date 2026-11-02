# JVM hotspots (JFR ExecutionSample)

Samples: 27082

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 5.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 5 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.35% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9838` |
| 9 | 2.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9363` |
| 10 | 1.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 12 | 1.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 18 | 1.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 21 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 23 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.86% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4033` |
| 25 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9354` |
| 31 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 32 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.52% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4379` |
| 36 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 39 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9791` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 41 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.42% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3906` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7265` |
| 46 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 53 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.37% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 55 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9783` |
| 60 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9456` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9353` |
| 6 | 2.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.42% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 2.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 2.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.91% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.55% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.50% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 1.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 18 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3939` |
| 24 | 0.88% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 25 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9446` |
| 26 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 28 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7141` |

GC pauses: 0, total 0 ms


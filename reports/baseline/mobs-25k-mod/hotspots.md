# JVM hotspots (JFR ExecutionSample)

Samples: 15883

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.36% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1786` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3572` |
| 3 | 2.37% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 4 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.05% | `java.util.ArrayList.elementData(int) line: 412` |
| 8 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3293` |
| 9 | 0.99% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14070` |
| 11 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9150` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3573` |
| 14 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3555` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5056` |
| 19 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1787` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9694` |
| 22 | 0.71% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.69% | `java.util.ArrayList.add(Object) line: 496` |
| 26 | 0.68% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 27 | 0.65% | `java.util.ArrayList.add(Object) line: 497` |
| 28 | 0.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 29 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 31 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9347` |
| 32 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6465` |
| 34 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3640` |
| 36 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.50% | `java.util.ArrayList.grow() line: 245` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9525` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9772` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13417` |
| 43 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14723` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10317` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6402` |
| 48 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 49 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 50 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.33% | `java.util.EnumMap.get(Object) line: 248` |
| 53 | 0.33% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 54 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 56 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3554` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 14904` |
| 60 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.20% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 6.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3293` |
| 3 | 2.93% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9787` |
| 8 | 1.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3641` |
| 9 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 10 | 1.20% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 11 | 1.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14348` |
| 12 | 1.05% | `java.util.ArrayList.get(int) line: 429` |
| 13 | 1.05% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1778` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9772` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 20 | 0.80% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 22 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.70% | `java.lang.ThreadLocal.get() line: 171` |
| 25 | 0.67% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9693` |
| 28 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9863` |
| 30 | 0.60% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms


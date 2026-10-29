# JVM hotspots (JFR ExecutionSample)

Samples: 13860

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 8 | 1.78% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 9 | 1.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 12 | 1.48% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks$RefField.getBool(Object) line: 4030` |
| 19 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.89% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.85% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 26 | 0.76% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 29 | 0.74% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8250` |
| 30 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.72% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3401` |
| 33 | 0.67% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8128` |
| 34 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.61% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 36 | 0.60% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 37 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3338` |
| 39 | 0.53% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.53% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6095` |
| 41 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 42 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 43 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.50% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 46 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 48 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2176` |
| 50 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 52 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 53 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 54 | 0.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 55 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 56 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 58 | 0.34% | `java.lang.ref.Reference.get() line: 361` |
| 59 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.98% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 3 | 2.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 8 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 1.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 13 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 15 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.02% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6091` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.navIdle$(PathNavigation) line: 8374` |
| 22 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 23 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 24 | 0.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.87% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.86% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2180` |
| 28 | 0.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4883` |
| 29 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 30 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges()` |

GC pauses: 0, total 0 ms


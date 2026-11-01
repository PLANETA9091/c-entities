# JVM hotspots (JFR ExecutionSample)

Samples: 13162

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 7 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 9 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 12 | 1.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.24% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4116` |
| 16 | 1.11% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 21 | 1.02% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 1.00% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.95% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 25 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.90% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 28 | 0.83% | `java.lang.ref.Reference.get() line: 361` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6889` |
| 30 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 32 | 0.75% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 33 | 0.75% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.75% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4053` |
| 35 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 36 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.61% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.53% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9044` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4361` |
| 43 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 46 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 53 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4206` |
| 56 | 0.33% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.38% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 2 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8849` |
| 4 | 2.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 5 | 2.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 2.06% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 2.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 9 | 1.93% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 15 | 1.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 1.12% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 20 | 1.02% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 21 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 1.01% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 23 | 1.01% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4362` |
| 24 | 1.00% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.99% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3412` |
| 26 | 0.96% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.95% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 29 | 0.94% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5677` |

GC pauses: 0, total 0 ms


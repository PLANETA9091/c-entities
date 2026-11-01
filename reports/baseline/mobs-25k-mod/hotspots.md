# JVM hotspots (JFR ExecutionSample)

Samples: 11903

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 7 | 2.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 11 | 1.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4146` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 15 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 1.18% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.05% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4083` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 20 | 0.99% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.97% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 23 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.91% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6973` |
| 25 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.84% | `java.lang.ref.Reference.get() line: 361` |
| 28 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 29 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.76% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.73% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9006` |
| 32 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 34 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.66% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.50% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4413` |
| 42 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9128` |
| 48 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 50 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 52 | 0.34% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 53 | 0.31% | `net.minecraft.core.component.PatchedDataComponentMap.equals(Object) line: 222` |
| 54 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4236` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8933` |
| 2 | 2.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 2.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 5 | 2.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 2.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 11 | 1.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.46% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 15 | 1.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 16 | 1.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 18 | 1.18% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 19 | 1.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 20 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 1.02% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 22 | 0.99% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4414` |
| 24 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 25 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.88% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.86% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6969` |
| 30 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges()` |

GC pauses: 0, total 0 ms


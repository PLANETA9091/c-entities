# JVM hotspots (JFR ExecutionSample)

Samples: 13516

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 3.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 3.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 9 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 14 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.12% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 1.10% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 20 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.99% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 22 | 0.97% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 23 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.90% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3538` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 26 | 0.90% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 27 | 0.90% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.89% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 29 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.78% | `java.lang.ref.Reference.get() line: 361` |
| 32 | 0.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 33 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.69% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 35 | 0.68% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8450` |
| 36 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.64% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.63% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 40 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.55% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 46 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3691` |
| 48 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3846` |
| 52 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 92` |
| 55 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.35% | `java.util.ImmutableCollections.listCopy(Collection) line: 191` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 58 | 0.34% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.33% | `java.lang.ThreadLocal.get() line: 171` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8255` |
| 3 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.03% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 1.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 12 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 14 | 1.46% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 2947` |
| 15 | 1.44% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 16 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 18 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 19 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.09% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.99% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.97% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.96% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 24 | 0.95% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 25 | 0.94% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.92% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.92% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 28 | 0.90% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5083` |
| 29 | 0.90% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |

GC pauses: 0, total 0 ms


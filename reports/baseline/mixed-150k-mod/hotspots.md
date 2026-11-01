# JVM hotspots (JFR ExecutionSample)

Samples: 28765

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 2 | 4.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 4 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 7 | 2.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 9 | 1.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 10 | 1.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 15 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 19 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 1.09% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 23 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.87% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.setDirty() line: 114` |
| 31 | 0.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9074` |
| 34 | 0.56% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3741` |
| 37 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.50% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 41 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.48% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 44 | 0.47% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 45 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 47 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 49 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 52 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.34% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4151` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 3 | 3.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.51% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 6 | 2.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.05% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 10 | 1.72% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 12 | 1.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 19 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.92% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6918` |
| 24 | 0.88% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 27 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6021` |
| 29 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms


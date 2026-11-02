# JVM hotspots (JFR ExecutionSample)

Samples: 11755

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 3.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 6 | 2.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4352` |
| 13 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 16 | 1.15% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 1.11% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 19 | 0.91% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4289` |
| 21 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7179` |
| 26 | 0.76% | `java.lang.ref.Reference.get() line: 361` |
| 27 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 31 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.60% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 38 | 0.45% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.44% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9212` |
| 42 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4619` |
| 45 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 48 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 52 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 54 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 92` |
| 55 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 56 | 0.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 57 | 0.30% | `java.util.ImmutableCollections.listCopy(Collection) line: 191` |
| 58 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4442` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 2 | 3.22% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 3 | 2.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 4 | 2.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 2.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.13% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 11 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 13 | 1.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 15 | 1.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 18 | 1.05% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4620` |
| 19 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 20 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 21 | 0.99% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5945` |
| 24 | 0.91% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 25 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 26 | 0.88% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.84% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7175` |
| 28 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4462` |
| 30 | 0.73% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 9820

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.71% | `java.lang.ref.Reference.get() line: 361` |
| 2 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.83% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 10 | 1.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4402` |
| 12 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 13 | 1.23% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9532` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6764` |
| 16 | 1.15% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7374` |
| 18 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.08% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4339` |
| 21 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6442` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 25 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 28 | 0.73% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 30 | 0.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6262` |
| 32 | 0.70% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4492` |
| 33 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6608` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10046` |
| 36 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.51% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.50% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 40 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.47% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9680` |
| 45 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 47 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4681` |
| 49 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 50 | 0.37% | `net.minecraft.core.component.PatchedDataComponentMap.equals(Object) line: 222` |
| 51 | 0.35% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 52 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 53 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.firstOf(Set) line: 6738` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6485` |
| 56 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 57 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6847` |
| 3 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 2.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6453` |
| 10 | 1.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.47% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 12 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9680` |
| 13 | 1.32% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.23% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 15 | 1.23% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4512` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6763` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7370` |
| 18 | 1.15% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4682` |
| 19 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 1.06% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 1.06% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6487` |
| 23 | 0.86% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.81% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4680` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 26 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |
| 28 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 29 | 0.73% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms


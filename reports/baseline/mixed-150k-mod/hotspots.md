# JVM hotspots (JFR ExecutionSample)

Samples: 35434

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 3 | 2.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 9 | 1.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.13% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 19 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.98% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 21 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.93% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 0.86% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 24 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 27 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 29 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 31 | 0.60% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 34 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 36 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 40 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.44% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 42 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 47 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10281` |
| 50 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 52 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 55 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 59 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 4 | 2.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 7 | 2.11% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 9 | 1.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.73% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 15 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 17 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 25 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7466` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 30 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms


# JVM hotspots (JFR ExecutionSample)

Samples: 28465

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 3 | 3.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 4 | 3.39% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 1.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.66% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 13 | 1.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.26% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 1.04% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 21 | 1.02% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 22 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2889` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 27 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 31 | 0.61% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 32 | 0.60% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 33 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 35 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2821` |
| 36 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 37 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 41 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 43 | 0.45% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 11326` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 45 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4632` |
| 47 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 48 | 0.40% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 50 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8250` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 55 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5129` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 59 | 0.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.31% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 4 | 3.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.39% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 6 | 2.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.39% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.86% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.67% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 12 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8126` |
| 17 | 1.16% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 19 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 1.06% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 1.04% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 24 | 1.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 26 | 0.85% | `java.util.HashMap.get(Object) line: 565` |
| 27 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4665` |
| 29 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.77% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

GC pauses: 0, total 0 ms


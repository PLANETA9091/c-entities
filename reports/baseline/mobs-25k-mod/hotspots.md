# JVM hotspots (JFR ExecutionSample)

Samples: 14640

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2669` |
| 2 | 7.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2400` |
| 4 | 1.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 1.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 8 | 1.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10723` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2398` |
| 14 | 0.97% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7249` |
| 17 | 0.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7317` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 23 | 0.65% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 26 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.58% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6915` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 32 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 35 | 0.53% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 36 | 0.52% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 37 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4833` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.46% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7859` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7093` |
| 44 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10816` |
| 47 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4770` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.getDeltaMovement() line: 5074` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 53 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 57 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5079` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2442` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 3 | 5.52% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 2.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.98% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 6 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7332` |
| 10 | 1.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 12 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2722` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 15 | 1.01% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 0.93% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2663` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7317` |
| 20 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7248` |
| 23 | 0.72% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.66% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 29 | 0.65% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 30 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms


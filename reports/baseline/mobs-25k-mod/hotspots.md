# JVM hotspots (JFR ExecutionSample)

Samples: 9972

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 2 | 2.97% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 2.13% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 1.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 10 | 1.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 14 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9614` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6501` |
| 18 | 1.04% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.94% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6679` |
| 21 | 0.92% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7445` |
| 22 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 24 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 25 | 0.83% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 29 | 0.68% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.64% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.63% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10128` |
| 35 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.51% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 41 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6528` |
| 45 | 0.39% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.36% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.firstOf(Set) line: 6809` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.36% | `net.minecraft.core.component.PatchedDataComponentMap.equals(Object) line: 222` |
| 53 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 55 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6502` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 60 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6918` |
| 3 | 3.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 4 | 2.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 5 | 2.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 2.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 9 | 1.93% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 12 | 1.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6834` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6991` |
| 15 | 1.15% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 1.04% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7441` |
| 20 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6833` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6663` |
| 30 | 0.71% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |

GC pauses: 0, total 0 ms


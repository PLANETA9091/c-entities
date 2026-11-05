# JVM hotspots (JFR ExecutionSample)

Samples: 11966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 4.18% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 3 | 3.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 9 | 1.60% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 10 | 1.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 13 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 16 | 1.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10314` |
| 18 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.99% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 22 | 0.89% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7901` |
| 23 | 0.84% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 24 | 0.78% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4771` |
| 25 | 0.77% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.74% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7135` |
| 28 | 0.67% | `java.util.TimSort.minRunLength(int) line: 405` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 30 | 0.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.61% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 33 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 38 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 39 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10862` |
| 41 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 45 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6984` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6958` |
| 50 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 53 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 54 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 55 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 56 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 57 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.29% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 59 | 0.28% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 60 | 0.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 4.53% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 4.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 3.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 5 | 3.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 6 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 2.15% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10497` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7290` |
| 14 | 1.18% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 16 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.07% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 18 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7447` |
| 22 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.84% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7897` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 25 | 0.77% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7119` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.67% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 231` |
| 29 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 30 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms


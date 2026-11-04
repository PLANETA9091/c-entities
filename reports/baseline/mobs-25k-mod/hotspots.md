# JVM hotspots (JFR ExecutionSample)

Samples: 13243

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 2.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 2.34% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.94% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 6 | 1.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.42% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 13 | 1.31% | `java.util.TimSort.minRunLength(int) line: 405` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 16 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 19 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.98% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7135` |
| 21 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 24 | 0.82% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4771` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 26 | 0.79% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.76% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7901` |
| 29 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.61% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 34 | 0.59% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.58% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 36 | 0.57% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 37 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6984` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10828` |
| 40 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.48% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 44 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 45 | 0.45% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 48 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 49 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 52 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 56 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.31% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5080` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.29% | `java.lang.String.hashCode() line: 2430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 4.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 2.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 4 | 2.74% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 5 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 2.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 10 | 1.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 12 | 1.61% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 1.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7290` |
| 14 | 1.44% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 15 | 1.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.31% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 231` |
| 17 | 1.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7447` |
| 18 | 1.16% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 1.03% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.98% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.92% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 24 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.80% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7119` |
| 29 | 0.79% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.76% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |

GC pauses: 0, total 0 ms


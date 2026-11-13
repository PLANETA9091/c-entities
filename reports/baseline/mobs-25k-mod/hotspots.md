# JVM hotspots (JFR ExecutionSample)

Samples: 21817

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.40% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 2 | 3.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2910` |
| 3 | 2.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2911` |
| 7 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.07% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8017` |
| 9 | 1.02% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.99% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 12 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7848` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2674` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7670` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8095` |
| 17 | 0.84% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 18 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7697` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11976` |
| 22 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 23 | 0.65% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 24 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.62% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.56% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 29 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5297` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5234` |
| 32 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 34 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.45% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8640` |
| 36 | 0.45% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 39 | 0.44% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 40 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 41 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.38% | `java.lang.String.hashCode() line: 2430` |
| 46 | 0.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 47 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 48 | 0.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2806` |
| 49 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 52 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12565` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 58 | 0.28% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.01% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 5.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2674` |
| 3 | 2.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 2.19% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 2.04% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8110` |
| 7 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8095` |
| 9 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.18% | `java.util.ArrayList.add(Object) line: 497` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8016` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8186` |
| 13 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.05% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7832` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12200` |
| 20 | 0.84% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.74% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.66% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.65% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 28 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.54% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms


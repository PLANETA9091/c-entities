# JVM hotspots (JFR ExecutionSample)

Samples: 14442

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 5.93% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 3 | 4.76% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.06% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10904` |
| 6 | 1.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 8 | 1.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 10 | 1.18% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10175` |
| 12 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10811` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 15 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.88% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 21 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.82% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.69% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 25 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.64% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.63% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 29 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.59% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 31 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 32 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7901` |
| 34 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 35 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 36 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.47% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7135` |
| 39 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 41 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4771` |
| 44 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10723` |
| 51 | 0.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 52 | 0.32% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 55 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.26% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.26% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 5.93% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 3 | 5.41% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 2.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.06% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 6 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10358` |
| 11 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 14 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 20 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 21 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 22 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 23 | 0.82% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 25 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7290` |
| 26 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.71% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 28 | 0.64% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.62% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |

GC pauses: 0, total 0 ms


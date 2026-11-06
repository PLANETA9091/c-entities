# JVM hotspots (JFR ExecutionSample)

Samples: 11693

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 2 | 4.73% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 3 | 4.60% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.09% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.09% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10392` |
| 12 | 1.01% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1292` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4834` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2399` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7304` |
| 21 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7382` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7135` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4771` |
| 25 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.58% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 27 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 28 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6957` |
| 30 | 0.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 33 | 0.48% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7927` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5080` |
| 41 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 42 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 44 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 49 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 50 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10940` |
| 51 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2410` |
| 55 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2630` |
| 58 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 60 | 0.26% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 2 | 5.19% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 4.73% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 4 | 3.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.72% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7382` |
| 7 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7397` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2664` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10575` |
| 14 | 1.14% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 15 | 1.09% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2723` |
| 19 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.92% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 21 | 0.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 22 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.80% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7473` |
| 27 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7303` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5081` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms


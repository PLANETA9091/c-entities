# JVM hotspots (JFR ExecutionSample)

Samples: 49343

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.99% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 18 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.51% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.45% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 24 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 30 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 33 | 0.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 35 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 36 | 0.35% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 37 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.34% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 40 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 45 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 49 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.27% | `java.util.Optional.get() line: 142` |
| 52 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 55 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 60 | 0.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 0.99% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.73% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.55% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 14 | 0.54% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 15 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 17 | 0.48% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 18 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 20 | 0.47% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.43% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.43% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 28 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.42% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms


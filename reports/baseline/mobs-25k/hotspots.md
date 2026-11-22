# JVM hotspots (JFR ExecutionSample)

Samples: 54642

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.60% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 0.97% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 8 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 0.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 20 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 24 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 25 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.34% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 29 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.33% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 31 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.30% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 36 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 37 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 41 | 0.25% | `java.util.ArrayDeque.size() line: 647` |
| 42 | 0.25% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 43 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.23% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.22% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.22% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 52 | 0.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 54 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 55 | 0.21% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.20% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 57 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.19% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 60 | 0.18% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 41.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.16% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 1.54% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.21% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 8 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 13 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 0.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 17 | 0.54% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.38% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.38% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 23 | 0.37% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 24 | 0.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.34% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 27 | 0.34% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 28 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.33% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$0(int, AtomicInteger, boolean, Vec3, Vec3, LongSet, boolean, AABB, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1896` |

GC pauses: 0, total 0 ms


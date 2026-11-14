# JVM hotspots (JFR ExecutionSample)

Samples: 48575

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.06% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.13% | `net.minecraft.world.entity.Mob.tick() line: 410` |
| 9 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.55% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 17 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.46% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 22 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 23 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 25 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 27 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 30 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.33% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 36 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.32% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 39 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 40 | 0.31% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 41 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 42 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 46 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 47 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.26% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 49 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.24% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.22% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 54 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 55 | 0.20% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 56 | 0.19% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 57 | 0.19% | `java.lang.String.hashCode() line: 2430` |
| 58 | 0.19% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.19% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 60 | 0.19% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 1.80% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.13% | `net.minecraft.world.entity.monster.zombie.Zombie.tick() line: 235` |
| 8 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.69% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 14 | 0.68% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 15 | 0.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.52% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 18 | 0.49% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 19 | 0.47% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 20 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 21 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 22 | 0.42% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 23 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 24 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.37% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.35% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 30 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms


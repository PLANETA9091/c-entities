# JVM hotspots (JFR ExecutionSample)

Samples: 58064

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.89% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.41% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 26 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 31 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 32 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.32% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 38 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 41 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 43 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.25% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 46 | 0.24% | `java.util.ArrayDeque.size() line: 647` |
| 47 | 0.24% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 50 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.21% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 54 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 55 | 0.20% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 56 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 58 | 0.19% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.19% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 60 | 0.19% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.89% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 6 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 12 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.58% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 17 | 0.47% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 20 | 0.42% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 21 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.41% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.41% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 24 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 25 | 0.38% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 26 | 0.37% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 27 | 0.36% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 28 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.35% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 30 | 0.35% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |

GC pauses: 0, total 0 ms


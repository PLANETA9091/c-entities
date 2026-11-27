# JVM hotspots (JFR ExecutionSample)

Samples: 60612

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.72% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.08% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 7 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.92% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.83% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 11 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.76% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getFluidState(BlockPos) line: 346` |
| 17 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.40% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 29 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 31 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 33 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 36 | 0.32% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 37 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 40 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 41 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 46 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 47 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 48 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.24% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 51 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 52 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 54 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 56 | 0.21% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 57 | 0.21% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 58 | 0.20% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4095` |
| 59 | 0.20% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 60 | 0.20% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.21% | `net.minecraft.world.entity.Mob.tick() line: 410` |
| 6 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.83% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 0.66% | `net.minecraft.world.level.Level.getFluidState(BlockPos) line: 1324` |
| 12 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.60% | `java.util.HashMap.containsKey(Object) line: 603` |
| 14 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.48% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 21 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.43% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 24 | 0.40% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 25 | 0.39% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.38% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.38% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.37% | `net.minecraft.util.Mth.degreesDifference(float, float) line: 256` |
| 29 | 0.36% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |

GC pauses: 0, total 0 ms


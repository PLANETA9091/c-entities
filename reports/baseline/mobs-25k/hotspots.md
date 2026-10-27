# JVM hotspots (JFR ExecutionSample)

Samples: 33248

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.97% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.57% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.50% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 26 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 30 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 32 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 33 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 34 | 0.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 35 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 40 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 42 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 43 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.27% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.24% | `java.util.Optional.isPresent() line: 154` |
| 51 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 54 | 0.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 55 | 0.21% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.21% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 58 | 0.20% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 59 | 0.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.97% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 0.75% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.67% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 15 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 16 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.58% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 21 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.54% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.52% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 24 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.46% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.43% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |

GC pauses: 0, total 0 ms

